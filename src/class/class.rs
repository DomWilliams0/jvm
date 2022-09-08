use std::cell::UnsafeCell;
use std::fmt::{Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread::ThreadId;

use itertools::Itertools;
use log::*;
use parking_lot::Mutex;

use cafebabe::mutf8::{mstr, StrExt};
use cafebabe::{
    attribute, AccessFlags, ClassAccessFlags, ClassError, FieldAccessFlags, MethodAccessFlags,
};

use crate::alloc::{vmref_eq, InternedString, NativeString, VmRef, WeakVmRef};
use crate::class::loader::current_thread;
use crate::class::object::Object;
use crate::class::{ClassLoader, WhichLoader};
use crate::constant_pool::RuntimeConstantPool;
use crate::error::{Throwable, Throwables, VmResult};
use crate::interpreter::{Frame, InterpreterError, NativeThunkHandle};
use crate::jni::NativeLibraries;
use crate::storage::{
    FieldDataType, FieldId, FieldStorage, FieldStorageLayout, FieldStorageLayoutBuilder,
};
use crate::thread;
use crate::types::{DataType, DataValue, MethodSignature, PrimitiveDataType, ReturnType};
use std::ffi::{CStr, CString};

// TODO when a ClassLoader is dropped, ensure all native libraries associated with it are freed too

pub struct Class {
    name: InternedString,
    class_type: ClassType,
    source_file: Option<NativeString>,
    state: LockedClassState,
    loader: WhichLoader,

    access_flags: ClassAccessFlags,

    /// java/lang/Class instance, initially NULL (!!!) because java/lang/Class hasn't been loaded,
    /// but is updated before any class is initialised and it is needed
    /// TODO weak reference for cyclic reference?
    class_object: MaybeUninit<VmRef<Object>>,
    #[cfg(debug_assertions)]
    class_object_init: bool,

    /// Only None for java/lang/Object
    super_class: Option<VmRef<Class>>,

    interfaces: Vec<VmRef<Class>>,
    fields: Vec<Field>,
    methods: Vec<VmRef<Method>>,

    constant_pool: RuntimeConstantPool,

    static_fields_layout: FieldStorageLayout,
    static_fields_values: FieldStorage,

    instance_fields_layout: FieldStorageLayout,
}

#[derive(Debug, Clone)]
pub enum ClassType {
    // TODO store dimensions
    Array(VmRef<Class>),
    Primitive(PrimitiveDataType),
    Normal,
}

#[derive(Debug, Copy, Clone)]
pub enum ClassState {
    /// verified and prepared but not initialized
    Uninitialised,
    /// being initialized by some particular thread
    Initialising(ThreadId),
    /// fully initialized and ready for use
    Initialised,
    /// in an erroneous state, perhaps because initialization was attempted and failed
    Error,
}

struct LockedClassState(UnsafeCell<ClassState>);

#[derive(Debug)]
pub struct Field {
    name: NativeString,
    desc: DataType<'static>,
    flags: FieldAccessFlags,
}

#[derive(Copy, Clone)]
pub enum FieldSearchType {
    Instance,
    Static,
}

/// Result of searching for a field in a class
pub enum FoundField {
    /// Field value is in the storage of the searched class
    InThisClass(FieldId),

    /// Field value is in the storage of a super class of the searched class (i.e. it's static)
    InOtherClass(FieldId, VmRef<Class>),
}

#[derive(Debug)]
pub enum MethodCode {
    /// Abstract, no code
    Abstract,

    /// Non-abstract method
    Java(attribute::Code),

    Native(Mutex<NativeCode>),
}

#[derive(Debug)]
pub enum NativeCode {
    Unbound,

    Bound(NativeFunction),

    /// Could not be bound
    FailedToBind,
}

pub type NativeInternalFn = fn(FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>>;

pub struct FunctionArgs<'a>(&'a mut [DataValue]);

#[derive(Clone)]
pub enum NativeFunction {
    /// Rust function
    Internal(NativeInternalFn),

    /// JNI style C function called directly from native, like JNI_OnLoad
    JniDirect(usize),

    Jni {
        ptr: usize,
        /// Populated on first use
        trampoline: Option<NativeThunkHandle>,
    },
}

#[derive(Debug)]
pub struct Method {
    name: NativeString,
    desc: NativeString,
    flags: MethodAccessFlags,
    /// Always initialised during linking
    class: MaybeUninit<VmRef<Class>>,

    // TODO arrayvec
    args: Vec<DataType<'static>>,
    return_type: ReturnType<'static>,

    code: MethodCode,
    attributes: Vec<attribute::OwnedAttribute>,
}

unsafe impl Sync for Method {}
unsafe impl Send for Method {}

pub enum MethodLookupResult {
    Found(VmRef<Method>),
    FoundMultiple,
    NotFound,
}

pub enum SuperIteration {
    KeepGoing,
    Stop,
}

struct ClassStaticFieldPrinter<'a>(&'a VmRef<Class>);

/// Short mangled name e.g. Java_com_me_Class_methodName
struct MangledMethodName(CString);

/// Long mangled name e.g. Java_com_me_Class_methodName__IL
struct MangledMethodNameLong(CString);

// TODO get classloader reference from tls instead of parameter

impl Class {
    pub fn link(
        expected_name: &mstr,
        loaded: cafebabe::ClassFile,
        loader: WhichLoader,
        classloader: &ClassLoader,
    ) -> VmResult<VmRef<Self>> {
        debug!("linking class {:?}", expected_name);
        // TODO this crashes in release builds, oops

        // check this is indeed the class we expected
        // TODO verify constant pool offsets so we can raise a single classformaterror then trust it
        let defined_class_name = loaded
            .this_class()
            .map_err(|_| Throwables::ClassFormatError)?;
        if defined_class_name != expected_name {
            warn!(
                "expected to load class {:?} but actually loaded {:?}",
                expected_name, defined_class_name
            );
            return Err(Throwables::NoClassDefFoundError);
        }

        let name = defined_class_name.to_owned();
        let source_file = match loaded.attribute::<attribute::SourceFile>() {
            Ok(src) => {
                trace!("source file: {:?}", src.0);
                Some(src.0)
            }
            Err(ClassError::Attribute(_)) => None,
            Err(e) => {
                warn!("failed to get sourcefile: {}", e);
                return Err(Throwables::ClassFormatError);
            }
        };

        // TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2

        // resolve superclass and interfaces
        let super_class = match loaded.super_class() {
            Ok(super_name) => {
                // ensure loaded
                trace!("super class: {:?}", super_name);
                let super_class =
                    classloader.load_class_caused_by(super_name, loader.clone(), &name)?;
                Some(super_class)
            }
            Err(ClassError::NoSuper) if name.as_bytes() == b"java/lang/Object" => {
                // the one exception, no super class expected
                trace!("no super class expected for java.lang.Object");
                None
            }
            Err(e) => {
                warn!("failed to get super class: {}", e);
                return Err(Throwables::ClassFormatError);
            }
        };

        let interfaces = {
            let mut vec = Vec::with_capacity(loaded.interface_count());
            for interface in loaded.interfaces() {
                let interface_name = match interface {
                    Ok(iface) => iface,
                    Err(e) => {
                        warn!("failed to get interface: {}", e);
                        return Err(Throwables::ClassFormatError);
                    }
                };

                let interface =
                    classloader.load_class_caused_by(interface_name, loader.clone(), &name)?;
                vec.push(interface);
            }

            trace!(
                "interfaces: {:?}",
                vec.iter().map(|iface| iface.name()).collect_vec()
            );
            vec
        };

        let methods = {
            let methods = loaded.methods();
            let mut vec = Vec::with_capacity(methods.len());
            for method in methods {
                let method: &cafebabe::MethodInfo = method; // ide

                let mut attributes = {
                    let mut vec = Vec::with_capacity(method.attributes.len());
                    for attr in method.attributes.iter() {
                        let attr = attr.to_owned(loaded.constant_pool()).map_err(|e| {
                            warn!("invalid attribute {:?}: {}", attr.name, e);
                            Throwables::ClassFormatError
                        })?;
                        vec.push(attr);
                    }

                    vec
                };
                let code = {
                    let idx = attributes
                        .iter()
                        .position(|a| matches!(a, attribute::OwnedAttribute::Code(_)));

                    match idx {
                        // abstract
                        _ if method.access_flags.contains(MethodAccessFlags::ABSTRACT) => {
                            if idx.is_some() {
                                warn!("abstract method {:?} has Code attribute", method.name);
                                return Err(Throwables::ClassFormatError);
                            }

                            MethodCode::Abstract
                        }

                        // native
                        _ if method.access_flags.contains(MethodAccessFlags::NATIVE) => {
                            if idx.is_some() {
                                warn!("native method {:?} has Code attribute", method.name);
                                return Err(Throwables::ClassFormatError);
                            }

                            MethodCode::Native(Mutex::new(NativeCode::Unbound))
                        }

                        // normal
                        Some(idx) => {
                            // pop from attributes list
                            let code = match attributes.swap_remove(idx) {
                                attribute::OwnedAttribute::Code(code) => code,
                                _ => unreachable!(),
                            };

                            MethodCode::Java(code)
                        }

                        None => {
                            warn!(
                                "missing Code attribute from non-abstrct non-native method {:?}",
                                method.name
                            );
                            return Err(Throwables::ClassFormatError);
                        }
                    }
                };

                let mut signature = MethodSignature::from_descriptor(method.descriptor);
                let args = signature.iter_args().map(|arg| arg.to_owned()).collect();
                if signature.errored() {
                    warn!("invalid method descriptor {:?}", method.descriptor);
                    return Err(Throwables::ClassFormatError);
                }

                trace!(
                    "method {:?} ({:?}), {:?}, {:?}",
                    method.name,
                    method.descriptor,
                    method.access_flags,
                    code
                );

                vec.push(VmRef::new(Method {
                    name: method.name.to_owned(),
                    desc: method.descriptor.to_owned(),
                    flags: method.access_flags,
                    class: MaybeUninit::zeroed(), // populated at the end
                    args,
                    return_type: signature.return_type().to_owned(),
                    code,
                    attributes,
                }))
            }
            vec
        };

        let fields = {
            let mut vec = Vec::with_capacity(loaded.fields().len());

            for field in loaded.fields() {
                let field: &cafebabe::FieldInfo = field; // ide
                let desc = DataType::from_descriptor(field.descriptor).ok_or_else(|| {
                    warn!(
                        "invalid field descriptor on {:?}: {:?}",
                        field.name, field.descriptor
                    );
                    Throwables::ClassFormatError
                })?;

                vec.push(Field {
                    name: field.name.to_owned(),
                    desc: desc.to_owned(),
                    flags: field.access_flags,
                })
            }

            vec
        };

        // initialise field layout
        let (static_fields_layout, instance_fields_layout) = {
            let mut static_builder = FieldStorageLayoutBuilder::empty();
            let mut instance_builder = FieldStorageLayoutBuilder::with_capacity(4, 16);

            // add fields from supers in resolution order
            // TODO different order for field layout than resolution? supers first to enable slicing? or not needed?
            // TODO no need to iterate interfaces when looking for instance fields, add separate iterator method
            Self::field_resolution_order_with(
                None, // no self yet
                &fields,
                &interfaces,
                super_class.as_ref(),
                |this_cls, fields| {
                    instance_builder.add_fields_from_class(fields.iter().filter_map(|f| {
                        if !f.flags.is_static() {
                            trace!("registering instance field {:?} (from {:?})", f, this_cls);
                            Some(FieldDataType::Present(f.desc.clone()))
                        } else {
                            None
                        }
                    }));

                    match this_cls {
                        None => {
                            // this is the first call with this class's fields, so they are present
                            static_builder.add_fields_from_class(fields.iter().filter_map(|f| {
                                if f.flags.is_static() {
                                    trace!("registering present static field {:?}", f);
                                    Some(FieldDataType::Present(f.desc.clone()))
                                } else {
                                    None
                                }
                            }));
                        }
                        Some(cls) => {
                            // static fields in super classes are not present
                            static_builder.add_fields_from_class(fields.iter().filter_map(|f| {
                                if f.flags.is_static() {
                                    trace!("registering non-present static field {:?} (present in {:?})", f, cls);
                                    Some(FieldDataType::NotPresent(f.desc.clone()))
                                } else {
                                    None
                                }
                            }));
                        }
                    }

                    SuperIteration::KeepGoing
                },
            );

            (static_builder.build(), instance_builder.build())
        };

        // preparation step - initialise static fields
        // TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors
        let static_fields_values = static_fields_layout.new_storage();

        let constant_pool =
            RuntimeConstantPool::from_cafebabe(loaded.constant_pool()).map_err(|e| {
                warn!("invalid constant pool: {}", e);
                Throwables::ClassFormatError
            })?;

        let access = loaded.access_flags();

        let class = Self::new(
            classloader,
            name,
            ClassType::Normal,
            source_file,
            loader,
            super_class,
            interfaces,
            fields,
            methods,
            access,
            constant_pool,
            instance_fields_layout,
            static_fields_layout,
            static_fields_values,
        );

        // fix up method class refs
        // safety: this is probably mostly safe, this is the only reference to the class but we need to
        // clone the Arc for each method, so it has to be immutable
        unsafe {
            let class_mut = &mut *(Arc::as_ptr(&class) as *mut Class);
            class_mut.methods.iter_mut().for_each(|m| {
                let m = Arc::get_mut(m).unwrap();
                m.class = MaybeUninit::new(class.clone());
            })
        }

        Ok(class)
    }

    pub fn new_array_class(
        name: &mstr,
        loader: WhichLoader,
        elem_cls: VmRef<Class>,
        classloader: &ClassLoader,
    ) -> VmResult<VmRef<Self>> {
        let super_class = classloader.get_bootstrap_class("java/lang/Object");

        // TODO Every array type implements the interfaces Cloneable and java.io.Serializable.
        let interfaces = Vec::new();

        let access_flags = {
            let flags = if let ClassType::Normal = elem_cls.class_type {
                let mut flags = elem_cls.access_flags;
                flags.remove(ClassAccessFlags::INTERFACE);
                flags
            } else {
                ClassAccessFlags::PUBLIC
            };

            flags | ClassAccessFlags::ABSTRACT | ClassAccessFlags::FINAL
        };

        let cls = Self::new(
            classloader,
            name.to_owned(),
            ClassType::Array(elem_cls),
            None,
            loader,
            Some(super_class),
            interfaces,
            Vec::new(),
            Vec::new(),
            access_flags,
            RuntimeConstantPool::empty(),
            FieldStorageLayout::empty(),
            FieldStorageLayout::empty(),
            FieldStorage::empty(),
        );

        Ok(cls)
    }

    pub fn new_primitive_class(
        name: &mstr,
        primitive: PrimitiveDataType,
        classloader: &ClassLoader,
    ) -> VmResult<VmRef<Self>> {
        let super_class = classloader.get_bootstrap_class("java/lang/Object");
        let access_flags =
            ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT | ClassAccessFlags::FINAL;

        let cls = Self::new(
            classloader,
            name.to_owned(),
            ClassType::Primitive(primitive),
            None,
            WhichLoader::Bootstrap,
            Some(super_class),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            access_flags,
            RuntimeConstantPool::empty(),
            FieldStorageLayout::empty(),
            FieldStorageLayout::empty(),
            FieldStorage::empty(),
        );

        Ok(cls)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        classloader: &ClassLoader,
        name: InternedString,
        class_type: ClassType,
        source_file: Option<NativeString>,
        loader: WhichLoader,
        super_class: Option<VmRef<Class>>,
        interfaces: Vec<VmRef<Class>>,
        fields: Vec<Field>,
        methods: Vec<VmRef<Method>>,
        access_flags: ClassAccessFlags,
        constant_pool: RuntimeConstantPool,
        instance_fields_layout: FieldStorageLayout,
        static_fields_layout: FieldStorageLayout,
        static_fields_values: FieldStorage,
    ) -> VmRef<Class> {
        assert!(super_class.is_none() == (name.as_bytes() == b"java/lang/Object"));

        let mut vm_class = VmRef::new(Self {
            name,
            class_type,
            access_flags,
            source_file,
            state: LockedClassState::default(),
            loader,
            class_object: MaybeUninit::uninit(),
            #[cfg(debug_assertions)]
            class_object_init: false,
            super_class,
            interfaces,
            methods,
            constant_pool,
            instance_fields_layout,
            static_fields_layout,
            static_fields_values,
            fields,
        });

        // alloc java/lang/Class if possible
        classloader.populate_class_vmdata(&mut vm_class);
        vm_class
    }

    fn find_method_no_dups(
        &self,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
    ) -> MethodLookupResult {
        let mut matching = self.methods.iter().filter(|m| {
            m.flags.contains(flags) && m.name.as_mstr() == name && m.desc.as_mstr() == desc
        });

        let first = matching.next();
        let next = matching.next();

        match (first, next) {
            (Some(m), None) => MethodLookupResult::Found(m.clone()),
            (Some(_), Some(_)) => MethodLookupResult::FoundMultiple,
            _ => MethodLookupResult::NotFound,
        }
    }

    pub fn find_class_constructor(&self) -> MethodLookupResult {
        self.find_method_no_dups(
            "<clinit>".as_mstr(),
            "()V".as_mstr(),
            MethodAccessFlags::STATIC,
        )
    }

    pub fn find_instance_constructor(&self, descriptor: &mstr) -> Option<VmRef<Method>> {
        debug_assert!(descriptor.to_utf8().ends_with('V'));

        self.find_method_in_this_only(
            "<init>".as_mstr(),
            descriptor,
            MethodAccessFlags::empty(),
            MethodAccessFlags::STATIC | MethodAccessFlags::ABSTRACT,
        )
    }

    pub fn find_method_by_id(&self, id: i32) -> Option<VmRef<Method>> {
        self.methods.get(id as usize).cloned()
    }

    pub fn find_callable_method(
        &self,
        name: &mstr,
        descriptor: &mstr,
        flags: MethodAccessFlags,
    ) -> VmResult<VmRef<Method>> {
        self.find_method_in_this_only(name, descriptor, flags, MethodAccessFlags::ABSTRACT)
            .ok_or(Throwables::Other("java/lang/NoSuchMethodError"))
    }

    pub fn find_method_in_this_only(
        &self,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
        antiflags: MethodAccessFlags,
    ) -> Option<VmRef<Method>> {
        debug_assert!(
            MethodSignature::is_valid(desc),
            "invalid method descriptor {:?}",
            desc
        );
        self.methods
            .iter()
            .find(|m| {
                m.flags.contains(flags)
                    && (m.flags - antiflags) == m.flags
                    && m.name.as_mstr() == name
                    && m.desc.as_mstr() == desc
            })
            .cloned()
    }

    /// Looks in self, super classes and super interfaces
    pub fn find_method_recursive_in_superclasses(
        &self,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
        antiflags: MethodAccessFlags,
    ) -> Option<VmRef<Method>> {
        // check self
        if let Some(method) = self.find_method_in_this_only(name, desc, flags, antiflags) {
            return Some(method);
        }

        // recurse on super
        if let Some(method) = self.super_class().and_then(|super_cls| {
            super_cls.find_method_recursive_in_superclasses(name, desc, flags, antiflags)
        }) {
            return Some(method);
        }

        // then superifaces if not yet found
        self.find_maximally_specific_method(name, desc, flags, antiflags)
    }

    pub fn find_constructors(
        &self,
        flags: MethodAccessFlags,
        antiflags: MethodAccessFlags,
    ) -> impl Iterator<Item = (usize, VmRef<Method>)> + '_ {
        // TODO search in super classes too?

        self.methods.iter().enumerate().filter_map(move |(i, m)| {
            (m.flags.contains(flags)
                && (m.flags - antiflags) == m.flags
                && m.is_instance_initializer())
            .then(|| (i, m.clone()))
        })
    }

    /// Looks in superinterfaces only
    pub fn find_maximally_specific_method(
        &self,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
        antiflags: MethodAccessFlags,
    ) -> Option<VmRef<Method>> {
        let antiflags = antiflags | MethodAccessFlags::PRIVATE | MethodAccessFlags::STATIC;

        let mut found = None;
        self.with_superinterfaces(|iface| {
            match iface.find_method_in_this_only(name, desc, flags, antiflags) {
                m @ Some(_) => {
                    found = m;
                    // TODO ensure there is only 1
                    SuperIteration::Stop
                }
                None => SuperIteration::KeepGoing,
            }
        });

        found
    }

    pub fn find_overriding_method(self: VmRef<Class>, method: &Method) -> Option<VmRef<Method>> {
        let can_override = |m: &Method| {
            let flags = m.flags;
            // TODO also this check, wtf does it mean:
            // mA is marked neither ACC_PUBLIC nor ACC_PROTECTED nor ACC_PRIVATE, and either (a)
            // the declaration of mA appears in the same run-time package as the declaration of mC,
            // or (b) if mA is declared in a class A and mC is declared in a class C, then there
            // exists a method mB declared in a class B such that C is a subclass of B and B is a
            // subclass of A and mC can override mB and mB can override mA.
            !flags.contains(MethodAccessFlags::PRIVATE)
                && !flags.contains(MethodAccessFlags::ABSTRACT)
                && flags.intersects(MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED)
                && m.name() == method.name()
                && m.descriptor() == method.descriptor()
        };

        // resolve in this class first
        if let Some(found) = self.find_method_in_this_only(
            method.name(),
            method.descriptor(),
            MethodAccessFlags::empty(),
            MethodAccessFlags::ABSTRACT,
        ) {
            return Some(found);
        }

        // recurse to find overridable
        let mut ret = None;

        self.with_supers(|cls| {
            for method in &cls.methods {
                if can_override(&method) {
                    ret = Some(method.to_owned());
                    return SuperIteration::Stop;
                }
            }

            SuperIteration::KeepGoing
        });

        ret
    }

    fn find_field_index_with(
        fields: &[Field],
        name: &mstr,
        desc: &DataType,
        search: FieldSearchType,
    ) -> Option<usize> {
        fields
            .iter()
            .filter(|f| search.matches(f.flags)) // index should skip non-instance/static fields
            .position(|f| f.desc == *desc && f.name.as_mstr() == name)
    }

    pub(in crate::class) fn find_field_index(
        &self,
        name: &mstr,
        desc: &DataType,
        search: FieldSearchType,
    ) -> Option<usize> {
        Self::find_field_index_with(&self.fields, name, desc, search)
    }

    fn find_field_recursive(
        self: &VmRef<Class>,
        name: &mstr,
        desc: &DataType,
        ty: FieldSearchType,
    ) -> Option<FoundField> {
        let mut found = None;
        let mut cls_idx = 0;

        enum FoundResult {
            /// (class, field)
            Static(VmRef<Class>, usize),
            /// (class index, field index)
            Instance(usize, usize),
        }

        trace!("searching for field {:?} in {:?}", name, self);
        self.field_resolution_order(|cls, fields| {
            let cls = cls.unwrap(); // always provided
            if let Some(idx) = Self::find_field_index_with(fields, name, desc, ty) {
                found = Some(match ty {
                    FieldSearchType::Instance => FoundResult::Instance(cls_idx, idx),
                    FieldSearchType::Static => FoundResult::Static(cls.clone(), idx),
                });
                SuperIteration::Stop
            } else {
                trace!("nope {} = {:?}", cls_idx, cls);
                cls_idx += 1;
                SuperIteration::KeepGoing
            }
        });

        match found? {
            FoundResult::Static(cls, field_idx) => {
                trace!(
                    "found static field {:?} in {:?} at field index {:?}",
                    name,
                    cls,
                    field_idx
                );

                let layout = &cls.static_fields_layout;

                // lookup present field in self, which was just resolved in this class
                let field_id = layout.get_self_id(field_idx).unwrap();

                Some(if vmref_eq(&cls, self) {
                    FoundField::InThisClass(field_id)
                } else {
                    FoundField::InOtherClass(field_id, cls)
                })
            }
            FoundResult::Instance(cls_idx, field_idx) => {
                trace!(
                    "found instance field {:?} in class index {:?} at field index {:?}",
                    name,
                    cls_idx,
                    field_idx
                );
                let layout = &self.instance_fields_layout;
                let field_id = layout.get_id(cls_idx, field_idx).unwrap();
                Some(FoundField::InThisClass(field_id))
            }
        }
    }

    pub fn find_instance_field_recursive(
        self: &VmRef<Class>,
        name: &mstr,
        desc: &DataType,
    ) -> Option<FieldId> {
        match self.find_field_recursive(name, desc, FieldSearchType::Instance)? {
            FoundField::InOtherClass(_, _) => unreachable!(), // all instance fields are present
            FoundField::InThisClass(field_id) => Some(field_id),
        }
    }

    pub fn find_static_field_recursive(
        self: &VmRef<Class>,
        name: &mstr,
        desc: &DataType,
    ) -> Option<FoundField> {
        self.find_field_recursive(name, desc, FieldSearchType::Static)
    }

    pub fn get_static_field(
        self: &VmRef<Class>,
        name: &'static str,
        desc: &'static str,
    ) -> DataValue {
        // TODO this is basically copied from getstatic, reuse instruction impl if possible
        let name = mstr::from_literal(name);
        let desc = DataType::from_descriptor(mstr::from_literal(desc)).expect("bad descriptor");

        // get field id
        let found = self
            .find_static_field_recursive(name, &desc)
            .expect("field not found");

        let (storage_class, field_id) = match found {
            FoundField::InThisClass(id) => (self.clone(), id),
            FoundField::InOtherClass(id, cls) => (cls, id),
        };

        self.ensure_init().expect("init failed");

        // get field value
        storage_class.static_fields().ensure_get(field_id)
    }

    pub fn is_instance_of(self: &VmRef<Class>, other: &VmRef<Class>) -> bool {
        if vmref_eq(self, other) {
            return true;
        }

        match self.class_type() {
            ClassType::Normal => {
                debug_assert!(!self.is_interface());

                let mut found = false;
                self.with_supers(|super_cls| {
                    if vmref_eq(super_cls, other) {
                        found = true;
                        SuperIteration::Stop
                    } else {
                        SuperIteration::KeepGoing
                    }
                });
                found
            }
            ClassType::Array(elem_cls) => {
                /*If S is an array type SC[], that is, an array of components of type SC, then:
                    If T is a class type, then T must be Object.
                    If T is an interface type, then T must be one of the interfaces implemented by arrays (JLS §4.10.3).
                    If T is an array type TC[], that is, an array of components of type TC, then one of the following must be true:
                        TC and SC are the same primitive type.
                        TC and SC are reference types, and type SC can be cast to TC by these run-time rules.
                */

                match other.class_type() {
                    ClassType::Normal if other.is_interface() => {
                        todo!("instanceof(array, interface type)")
                    }
                    ClassType::Normal => other.name() == "Ljava/lang/Object;".as_mstr(),
                    ClassType::Array(other_elem) => {
                        if let Some(arr_prim) = elem_cls.class_type().as_primitive() {
                            if let Some(other_prim) = other_elem.class_type().as_primitive() {
                                return arr_prim == other_prim;
                            }

                            return false;
                        } else {
                            assert!(!elem_cls.class_type.is_array());
                            elem_cls.is_instance_of(other_elem)
                        }
                    }
                    ClassType::Primitive(_) => false,
                }
            }

            ClassType::Primitive(_) => unreachable!(),
        }
    }

    /// Self is an array class (dest), other is the source array to copy from
    pub fn can_array_be_copied_to(&self, other: &VmRef<Class>) -> bool {
        let (src_arr_elem, dst_arr_elem) = match self
            .class_type()
            .array_class()
            .zip(other.class_type().array_class())
        {
            Some(tup) => tup,
            None => return false,
        };

        dst_arr_elem.can_array_elem_be_assigned_to(src_arr_elem)
    }

    /// Self is an array class, other is the element to assign (can be null)
    pub fn can_array_be_assigned_to(&self, other: &VmRef<Object>) -> bool {
        let (src_arr_elem, dst_elem) = match (self.class_type().array_class(), other.class()) {
            (Some(arr), None) => {
                // null only for reference types?
                if arr.class_type().as_primitive().is_some() {
                    warn!("todo: assign null to primitive?");
                    return false;
                }

                return true;
            }
            (Some(arr), Some(val)) => (arr, val),
            _ => return false,
        };

        dst_elem.can_array_elem_be_assigned_to(src_arr_elem)
    }

    /// Self is an array element class (dest), other is the source array element class to copy from
    fn can_array_elem_be_assigned_to(self: &VmRef<Class>, src_arr_elem: &VmRef<Class>) -> bool {
        let dst_arr_elem = self;
        if vmref_eq(src_arr_elem, dst_arr_elem) {
            true
        } else {
            trace!(
                "can array of {:?} be assigned to by {:?}",
                src_arr_elem.class_type(),
                dst_arr_elem.class_type()
            );
            match (src_arr_elem.class_type(), dst_arr_elem.class_type()) {
                (ClassType::Array(_), _) | (_, ClassType::Array(_)) => {
                    unreachable!("nested arrays?")
                }

                (ClassType::Primitive(a), ClassType::Primitive(b)) => a == b,
                (ClassType::Normal, ClassType::Normal) => {
                    // succeed if one is object
                    if src_arr_elem.name() == "java/lang/Object".as_mstr()
                        || dst_arr_elem.name() == "java/lang/Object".as_mstr()
                    {
                        true
                    } else {
                        todo!(
                            "check superclasses for assignment of {} = {}",
                            dst_arr_elem.name(),
                            src_arr_elem.name()
                        )
                    }
                }
                _ => false,
            }
        }
    }

    /*    fn implements(self: &VmRef<Class>, iface: &VmRef<Class>) -> bool {
            vmref_eq(self, iface)
                || self
                    .interfaces
                    .iter()
                    .any(|implemented_iface| vmref_eq(implemented_iface, iface))
        }

        pub fn extends(self: &VmRef<Class>, cls: &VmRef<Class>) -> bool {
            let mut current = Some(self);
            while let Some(this_cls) = current {
                if vmref_eq(this_cls, cls) {
                    return true;
                }

                current = this_cls.super_class();
            }

            false
        }
    */
    /// Gross
    pub fn extends_by_name(self: &VmRef<Class>, cls: &mstr) -> bool {
        let mut current = Some(self);
        while let Some(this_cls) = current {
            if this_cls.name() == cls {
                return true;
            }

            current = this_cls.super_class();
        }

        false
    }

    pub fn name(&self) -> &mstr {
        &self.name
    }

    pub const fn constant_pool(&self) -> &RuntimeConstantPool {
        &self.constant_pool
    }

    pub const fn loader(&self) -> &WhichLoader {
        &self.loader
    }

    pub fn super_class(&self) -> Option<&VmRef<Class>> {
        self.super_class.as_ref()
    }

    pub fn class_type(&self) -> &ClassType {
        &self.class_type
    }

    pub fn class_object(&self) -> &VmRef<Object> {
        // safety: java/lang/Class is loaded and this is populated before any class is initialised
        #[cfg(debug_assertions)]
        {
            assert!(self.class_object_init);
        }
        unsafe { self.class_object.assume_init_ref() }
    }

    /// class_cls: java/lang/Class class to be instantiated in this method
    pub(in crate::class) fn init_class_object(self: &mut VmRef<Class>, class_cls: VmRef<Class>) {
        // allocate class instance
        let cls_object = VmRef::new(Object::new(class_cls));

        // set vmdata field on new class instance
        let (_, field_id) = cls_object.vmdata();
        let fields = cls_object.fields().unwrap();
        fields.ensure_set(field_id, DataValue::VmDataClass(self.clone()));

        // point class_object field to class instance
        // TODO use Arc::get_mut_unchecked instead when stable
        unsafe {
            let self_mut = Arc::get_mut_unchecked(self);
            #[cfg(debug_assertions)]
            {
                assert!(!std::mem::replace(&mut self_mut.class_object_init, true));
            }
            self_mut.class_object.write(cls_object);
        }
    }

    /// Class object monitor must be held!!
    fn get_state(&self) -> ClassState {
        debug_assert!(self.class_object().monitor.is_locked());
        // safety: asserted monitor is locked
        unsafe { *self.state.0.get() }
    }

    /// Class object monitor must be held!!
    fn set_state(&self, new_state: ClassState) {
        debug_assert!(self.class_object().monitor.is_locked());
        // safety: asserted monitor is locked
        unsafe {
            *self.state.0.get() = new_state;
        }
    }

    pub fn needs_init(&self) -> bool {
        let _monitor = self.class_object().enter_monitor();
        matches!(self.get_state(), ClassState::Uninitialised)
    }

    pub fn ensure_init(self: &Arc<Class>) -> VmResult<()> {
        // synchronise on initialisation lock
        let mut monitor = self.class_object().enter_monitor();

        match self.get_state() {
            ClassState::Error => Err(Throwables::NoClassDefFoundError),
            ClassState::Initialised => Ok(()),
            ClassState::Initialising(thread) => {
                if thread == current_thread() {
                    // recursive request
                    Ok(())
                } else {
                    // another thread is initialising, block until initialised
                    debug!(
                        "blocking thread {:?} until class {:?} is initialised",
                        current_thread(),
                        self.name
                    );
                    while let ClassState::Initialising(_) = self.get_state() {
                        monitor.wait();
                    }

                    debug!("thread {:?} unblocked", current_thread());

                    match self.get_state() {
                        ClassState::Error => Err(Throwables::NoClassDefFoundError),
                        ClassState::Initialised => Ok(()),
                        _ => unreachable!(),
                    }
                }
            }
            ClassState::Uninitialised => {
                // this thread's time to shine
                self.set_state(ClassState::Initialising(current_thread()));

                // release monitor
                drop(monitor);

                // TODO initialise final static fields from ConstantValue attrs

                // recursively initialise super classes only if this is a class
                let mut result = Ok(());
                if !self.is_interface() {
                    if let Some(super_class) = self.super_class() {
                        trace!(
                            "initialising super class of {}: {}",
                            self.name(),
                            super_class.name()
                        );

                        result = super_class.ensure_init().map_err(|e| {
                            debug!("super class initialisation failed: {:?}", e);
                            e
                        });
                    }

                    // initialise all superinterfaces that have at least one non-abstract and
                    // non-static method

                    result = result.and_then(|_| {
                        let mut result = Ok(());
                        self.with_superinterfaces(|iface| {
                            let should_init = iface.methods.iter().any(|m| {
                                let antiflags =
                                    MethodAccessFlags::STATIC | MethodAccessFlags::ABSTRACT;
                                (m.flags - antiflags) == m.flags
                            });

                            let mut iter_result = SuperIteration::KeepGoing;
                            if should_init {
                                trace!(
                                    "initialising super interface of {}: {}",
                                    self.name(),
                                    iface.name()
                                );

                                if let Err(e) = iface.ensure_init() {
                                    debug!("super interface initialisation failed: {:?}", e);
                                    result = Err(e);
                                    iter_result = SuperIteration::Stop;
                                }
                            }

                            iter_result
                        });
                        result
                    });
                }

                // run class constructor
                result = result.and_then(|_| {
                    match self.find_class_constructor() {
                        MethodLookupResult::FoundMultiple => {
                            warn!("class has multiple static constructors");
                            return Err(Throwables::ClassFormatError);
                        }
                        MethodLookupResult::NotFound => { /* no problem */ }
                        MethodLookupResult::Found(m) => {
                            debug!("running static constructor for {:?}", self.name);

                            let thread = thread::get();
                            let interpreter = thread.interpreter();
                            let result = Frame::new_no_args(m).and_then(|frame| {
                                interpreter.execute_frame(frame).map_err(|exc| {
                                    // TODO wrap exception here and return the proper type
                                    warn!("exception raised in static constructor: {:?}", exc);

                                    thread::get().set_exception(exc);
                                    InterpreterError::ExceptionRaised(Throwables::ClassFormatError)
                                })
                            });

                            if let Err(err) = result {
                                warn!("static constructor failed: {}", err);
                                // TODO proper exception type here
                                return Err(Throwables::ClassFormatError);
                            }

                            trace!("initialized class: {:?}", ClassStaticFieldPrinter(&*self))
                        }
                    }

                    Ok(())
                });

                // obtain monitor for updating state
                let monitor = self.class_object().enter_monitor();
                match result {
                    Err(e) => {
                        debug!("class initialisation failed: {:?}", e);

                        // update state
                        self.set_state(ClassState::Error);

                        // notify all threads
                        monitor.notify_all();

                        // TODO specific exception type e.g. ExceptionInInitializerError
                        Err(e)
                    }
                    Ok(()) => {
                        self.set_state(ClassState::Initialised);
                        monitor.notify_all();
                        Ok(())
                    }
                }
            }
        }
    }

    /// Recurses superclass then all superinterfaces
    fn with_supers(&self, mut f: impl FnMut(&VmRef<Class>) -> SuperIteration) {
        self.__with_supers_recurse(&mut f);
    }

    fn __with_supers_recurse(
        &self,
        f: &mut impl FnMut(&VmRef<Class>) -> SuperIteration,
    ) -> SuperIteration {
        let mut keep_going = SuperIteration::KeepGoing;

        // super first
        if let Some(super_class) = self.super_class.clone() {
            keep_going = f(&super_class);
            if matches!(keep_going, SuperIteration::KeepGoing) {
                keep_going = Self::__with_supers_recurse(&super_class, f);
            }
        }

        // then recurse on direct superinterfaces
        let mut ifaces = self.interfaces.iter();
        while matches!(keep_going, SuperIteration::KeepGoing) {
            match ifaces.next() {
                Some(iface) => {
                    keep_going = f(iface);

                    if matches!(keep_going, SuperIteration::KeepGoing) {
                        keep_going = Self::__with_supers_recurse(iface, f);
                    }
                }
                None => break,
            }
        }

        keep_going
    }

    /// Only recurses on superinterfaces
    fn with_superinterfaces(&self, mut f: impl FnMut(&VmRef<Class>) -> SuperIteration) {
        self.__with_superinterfaces_recurse(&mut f);
    }

    fn __with_superinterfaces_recurse(
        &self,
        f: &mut impl FnMut(&VmRef<Class>) -> SuperIteration,
    ) -> SuperIteration {
        let mut keep_going = SuperIteration::KeepGoing;

        let mut ifaces = self.interfaces.iter();
        while matches!(keep_going, SuperIteration::KeepGoing) {
            match ifaces.next() {
                Some(iface) => {
                    keep_going = f(iface);

                    if matches!(keep_going, SuperIteration::KeepGoing) {
                        keep_going = Self::__with_supers_recurse(iface, f);
                    }
                }
                None => break,
            }
        }

        keep_going
    }

    pub(in crate::class) fn field_resolution_order(
        self: &VmRef<Class>,
        mut f: impl FnMut(Option<&VmRef<Class>>, &[Field]) -> SuperIteration,
    ) {
        Self::__field_resolution_order_recurse(
            Some(self),
            &self.fields,
            &self.interfaces,
            self.super_class.as_ref(),
            &mut f,
        );
    }

    fn field_resolution_order_with(
        this_class: Option<&VmRef<Class>>,
        fields: &[Field],
        interfaces: &[VmRef<Class>],
        super_class: Option<&VmRef<Class>>,
        mut f: impl FnMut(Option<&VmRef<Class>>, &[Field]) -> SuperIteration,
    ) {
        Self::__field_resolution_order_recurse(this_class, fields, interfaces, super_class, &mut f);
    }

    fn __field_resolution_order_recurse(
        this_class: Option<&VmRef<Class>>,
        fields: &[Field],
        interfaces: &[VmRef<Class>],
        super_class: Option<&VmRef<Class>>,
        f: &mut impl FnMut(Option<&VmRef<Class>>, &[Field]) -> SuperIteration,
    ) -> SuperIteration {
        let mut keep_going;

        // own fields first
        keep_going = f(this_class, fields);

        // then recurse on direct super interfaces
        let mut ifaces = interfaces.iter();
        while matches!(keep_going, SuperIteration::KeepGoing) {
            match ifaces.next() {
                Some(iface) => {
                    keep_going = Self::__field_resolution_order_recurse(
                        Some(iface),
                        &iface.fields,
                        &iface.interfaces,
                        iface.super_class.as_ref(),
                        f,
                    );
                }
                None => break,
            }
        }

        // then recurse on super
        if matches!(keep_going, SuperIteration::KeepGoing) {
            if let Some(super_class) = super_class {
                return Self::__field_resolution_order_recurse(
                    Some(super_class),
                    &super_class.fields,
                    &super_class.interfaces,
                    super_class.super_class.as_ref(),
                    f,
                );
            }
        }

        keep_going
    }

    pub fn static_fields(&self) -> &FieldStorage {
        &self.static_fields_values
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags.contains(ClassAccessFlags::INTERFACE)
    }

    pub fn flags(&self) -> ClassAccessFlags {
        self.access_flags
    }

    pub fn instance_fields_layout(&self) -> &FieldStorageLayout {
        &self.instance_fields_layout
    }

    pub fn static_fields_layout(&self) -> &FieldStorageLayout {
        &self.static_fields_layout
    }

    pub fn ensure_method_bound(&self, method: &Method) -> Result<(), InterpreterError> {
        let mut guard = match &method.code {
            MethodCode::Native(native) => {
                let guard = native.lock();
                match *guard {
                    NativeCode::Unbound => guard,
                    _ => return Ok(()),
                }
            }
            _ => return Ok(()),
        };

        // native method was not already bound (e.g. by bootstrap preload), so fallback to resolving
        // it as a JNI method

        debug!("binding native method {}", method);

        let name = method.mangled_native_name();
        let thread = thread::get();
        let native_libs: &mut NativeLibraries = &mut *thread.global().native_libraries_mut();

        // try short name first, then long name
        let ptr = native_libs
            .resolve_symbol(name.as_ref())
            .or_else(|| native_libs.resolve_symbol(name.into_long(method.descriptor()).as_ref()))
            .ok_or_else(|| InterpreterError::NativeMethodNotFound {
                class: method.class().clone(),
                name: method.name.clone(),
                desc: method.desc.clone(),
            })?;

        debug!("found native method at {:?}", ptr);

        // bind method
        *guard = NativeCode::Bound(NativeFunction::Jni {
            ptr: ptr as usize,
            trampoline: None,
        });
        Ok(())
    }

    pub fn bind_internal_method(&self, method: &Method, function: NativeInternalFn) -> bool {
        if let MethodCode::Native(native) = &method.code {
            let mut guard = native.lock();
            if let NativeCode::Unbound = *guard {
                // *guard = NativeCode::Bound(CompiledCode::new(CodeType::Trampoline(fn_ptr)));
                *guard = NativeCode::Bound(NativeFunction::Internal(function));
                debug!("bound {} to {:?}", method, *guard);
                return true;
            }
        }

        false
    }

    pub fn iter_static_fields(
        self: &VmRef<Class>,
        mut per_field: impl FnMut(&Field, DataValue) -> SuperIteration,
    ) {
        let mut cls_idx = 0;
        self.field_resolution_order(|cls_opt, fields| {
            let cls = cls_opt.unwrap_or(self);
            let layout = &cls.static_fields_layout;
            let field_storage = &cls.static_fields_values;

            for (i, field) in fields.iter().filter(|f| f.flags().is_static()).enumerate() {
                let field_id = layout.get_self_id(i).unwrap();
                let val = field_storage.ensure_get(field_id);

                if let SuperIteration::Stop = per_field(field, val) {
                    break;
                }
            }

            cls_idx += 1;
            SuperIteration::KeepGoing
        });
    }

    pub fn print_static_fields<'a>(self: &'a VmRef<Class>) -> impl Debug + 'a {
        ClassStaticFieldPrinter(self)
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class({})", self.name())
    }
}

impl Debug for ClassStaticFieldPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cls = self.0;
        write!(f, "Static fields for {:?}: ", cls)?;

        let mut result = Ok(());
        cls.iter_static_fields(|field, val| {
            result = write!(
                f,
                "\n * {} ({:?} {:?}) => {:?}",
                field.name(),
                field.desc(),
                field.flags(),
                val
            );
            if result.is_err() {
                SuperIteration::Stop
            } else {
                SuperIteration::KeepGoing
            }
        });

        result
    }
}

impl Method {
    pub fn code(&self) -> &MethodCode {
        &self.code
    }

    pub fn name(&self) -> &mstr {
        &self.name
    }

    pub fn descriptor(&self) -> &mstr {
        &self.desc
    }

    pub fn args(&self) -> &[DataType] {
        &self.args
    }

    pub fn flags(&self) -> MethodAccessFlags {
        self.flags
    }

    pub fn return_type(&self) -> &ReturnType {
        &self.return_type
    }

    pub fn is_instance_initializer(&self) -> bool {
        self.name().as_bytes() == b"<init>"
    }

    pub fn is_class_initializer(&self) -> bool {
        self.name().as_bytes() == b"<clinit>"
    }

    pub fn class(&self) -> &VmRef<Class> {
        // safety: always initialised in Class::link
        unsafe { &*self.class.as_ptr() }
    }

    fn mangled_native_name(&self) -> MangledMethodName {
        // TODO cache mangled name in the method
        MangledMethodName::new(self.class().name(), self.name())
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}::{}",
            self.class().name(),
            self.name(),
            self.descriptor()
        )
    }
}

impl Drop for Method {
    fn drop(&mut self) {
        // safety: always initialised in Class::link, and needs to be manually dropped
        unsafe {
            let class_ptr = self.class.as_mut_ptr();
            class_ptr.drop_in_place()
        }
    }
}

impl Field {
    pub fn name(&self) -> &mstr {
        &self.name
    }
    pub fn desc(&self) -> &DataType<'static> {
        &self.desc
    }
    pub fn flags(&self) -> FieldAccessFlags {
        self.flags
    }
}

impl MethodLookupResult {
    fn ok(self) -> Option<VmRef<Method>> {
        if let MethodLookupResult::Found(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

impl ClassType {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_primitive(&self) -> Option<PrimitiveDataType> {
        match self {
            Self::Primitive(p) => Some(*p),
            _ => None,
        }
    }
    pub fn array_class(&self) -> Option<&VmRef<Class>> {
        match self {
            Self::Array(cls) => Some(cls),
            _ => None,
        }
    }
}

impl FieldSearchType {
    pub fn matches(&self, flags: FieldAccessFlags) -> bool {
        let is_static = matches!(self, Self::Static);
        is_static == flags.is_static()
    }
}

impl NativeFunction {
    pub fn ensure_native_trampoline(&mut self, method: &Method) -> Result<(), InterpreterError> {
        let (fn_ptr, trampoline) = match self {
            NativeFunction::Jni { ptr, trampoline } if trampoline.is_none() => (*ptr, trampoline),
            _ => return Ok(()),
        };

        trace!("generating native trampoline for method {}", method);

        let thread = thread::get();
        let mut allocator = thread.global().native_thunks_mut();

        let handle = allocator.new_for_method(method, fn_ptr)?;
        *trampoline = Some(handle);
        Ok(())
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        use NativeFunction::*;
        match (self, other) {
            (Internal(a), Internal(b)) => std::ptr::eq(a, b),
            (JniDirect(a), JniDirect(b)) => a == b,
            _ => false,
        }
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ptr = match self {
            NativeFunction::Internal(f) => f as *const _ as usize,
            NativeFunction::JniDirect(f) | NativeFunction::Jni { ptr: f, .. } => *f,
        };
        write!(f, "NativeFunction({:#x})", ptr)
    }
}

impl<'a> From<&'a mut [DataValue]> for FunctionArgs<'a> {
    fn from(args: &'a mut [DataValue]) -> Self {
        Self(args)
    }
}

impl<'a> FunctionArgs<'a> {
    pub fn take(&mut self, idx: usize) -> DataValue {
        self.try_take(idx)
            .expect("bad arg index, should have been verified")
    }

    pub fn try_take(&mut self, idx: usize) -> Option<DataValue> {
        let val = self.0.get_mut(idx)?;
        Some(std::mem::replace(val, DataValue::Boolean(false)))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Reverse order
    pub fn take_all(self) -> impl Iterator<Item = DataValue> + 'a {
        self.0
            .iter_mut()
            .map(|val| std::mem::replace(val, DataValue::Boolean(false)))
    }
}

impl Default for LockedClassState {
    fn default() -> Self {
        LockedClassState(UnsafeCell::new(ClassState::Uninitialised))
    }
}

/// Will only be accessed when the class monitor is held
unsafe impl Sync for LockedClassState {}

impl Debug for LockedClassState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockedClassState")
    }
}

impl MangledMethodName {
    fn new(class_name: &mstr, method_name: &mstr) -> Self {
        let mut mangled = {
            // +6 for Java_ and other _
            let estimated_len = class_name.len() + method_name.len() + 6;
            Vec::with_capacity(estimated_len)
        };

        mangled.extend(b"Java_");
        class_name
            .to_utf8()
            .chars()
            .for_each(|c| Self::mangle_char(c, &mut mangled));
        mangled.push(b'_');
        method_name
            .to_utf8()
            .chars()
            .for_each(|c| Self::mangle_char(c, &mut mangled));

        // safety: mangled
        debug_assert!(std::str::from_utf8(&mangled).is_ok());
        MangledMethodName(unsafe { CString::from_vec_unchecked(mangled) })
    }

    /// desc assumed valid
    fn into_long(self, desc: &mstr) -> MangledMethodNameLong {
        debug_assert!(MethodSignature::is_valid(desc));

        let params = {
            let start = 1;
            let end = desc
                .as_bytes()
                .iter()
                .rev()
                .position(|b| *b == b')')
                .unwrap(); // assumed valid
            let end = desc.len() - 1 - end; // found in reverse
            mstr::from_mutf8(&desc.as_bytes()[start..end])
        };

        let mut string = self.0.into_bytes();

        string.extend(b"__");
        params
            .to_utf8()
            .chars()
            .for_each(|c| Self::mangle_char(c, &mut string));

        // safety: mangled
        debug_assert!(std::str::from_utf8(&string).is_ok());
        MangledMethodNameLong(unsafe { CString::from_vec_unchecked(string) })
    }

    fn mangle_char(c: char, out: &mut Vec<u8>) {
        let mut buf = [0; 6];
        match c {
            '/' => out.push(b'_'),
            '_' => out.extend(b"_1"),
            ';' => out.extend(b"_2"),
            '[' => out.extend(b"_3"),
            c if c.is_ascii() => {
                let str = c.encode_utf8(&mut buf);
                out.extend(str.as_bytes());
            }
            c => {
                use std::io::Write;
                let _ = write!(&mut buf[..], "_0{:04x}", c as u32);
                out.extend(&buf);
            }
        }
    }
}

impl AsRef<CStr> for MangledMethodName {
    fn as_ref(&self) -> &CStr {
        self.0.as_ref()
    }
}

impl AsRef<CStr> for MangledMethodNameLong {
    fn as_ref(&self) -> &CStr {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Jvm, JvmArgs};
    use cafebabe::mutf8::MString;
    use std::path::PathBuf;

    fn test_jvm() -> Jvm {
        macro_rules! var {
            ($var:expr) => {
                std::env::var($var).expect(std::concat!("missing env var ", $var))
            };
        }
        let test_cases = {
            let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root.push("test-cases");
            String::from(root.to_string_lossy())
        };

        let bootpath = {
            let gnu_classpath = var!("JVM_BOOTCLASSPATH");
            format!("{}:{}", gnu_classpath, test_cases)
        };

        let args = vec![
            "<unused main>",
            "--XXnosystemclassloader",
            "--Xbootclasspath",
            &bootpath,
            "--cp",
            &test_cases,
        ];
        let args = JvmArgs::parse(args.into_iter().map(String::from)).expect("bad args");
        Jvm::new(args).expect("init failed")
    }

    fn test_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Trace)
            .filter_module("cafebabe", LevelFilter::Info)
            .try_init();
    }

    fn get_static_field(class: &VmRef<Class>, name: &'static str, desc: &'static str) -> DataValue {
        // TODO this is basically copied from getstatic, reuse instruction impl if possible

        let name = mstr::from_literal(name);
        let desc = DataType::from_descriptor(mstr::from_literal(desc)).expect("bad descriptor");

        // get field id
        let found = class
            .find_static_field_recursive(name, &desc)
            .expect("field not found");

        let (storage_class, field_id) = match found {
            FoundField::InThisClass(id) => (class.clone(), id),
            FoundField::InOtherClass(id, cls) => (cls, id),
        };

        class.ensure_init().expect("init failed");

        // get field value
        info!(
            "searched {:?}, found id {:?} in {:?}",
            class, field_id, storage_class
        );
        storage_class.static_fields().ensure_get(field_id)
    }

    fn get_class(name: &'static str) -> VmRef<Class> {
        let thread = thread::get();
        let classloader = thread.global().class_loader();
        classloader
            .load_class(mstr::from_literal(name), WhichLoader::Bootstrap)
            .unwrap_or_else(|err| panic!("failed to load class {:?}: {}", name, err.symbol()))
    }

    #[test]
    fn static_field_inheritance_get() {
        test_logging();
        let _jvm = test_jvm();

        // TODO compile java source code at test time

        let outer = get_class("Inheritance");
        let inner1 = get_class("Inheritance$Inner");
        let inner2 = get_class("Inheritance$InnerDeeper");

        // own fields
        assert_eq!(get_static_field(&outer, "FIVE", "I"), DataValue::Int(5));
        assert_eq!(get_static_field(&inner1, "SIX", "I"), DataValue::Int(6));

        // inherited from super class
        assert_eq!(get_static_field(&inner1, "FIVE", "I"), DataValue::Int(5));
        assert_eq!(get_static_field(&inner2, "SIX", "I"), DataValue::Int(6));

        // inherited from super super class
        assert_eq!(get_static_field(&inner2, "FIVE", "I"), DataValue::Int(5));
    }

    #[test]
    fn static_field_iter() {
        test_logging();
        let _jvm = test_jvm();

        let inner1 = get_class("Inheritance$Inner");
        let inner2 = get_class("Inheritance$InnerDeeper");

        fn check_statics(
            cls: &VmRef<Class>,
            expected: Vec<(&'static str, &'static str, DataValue)>,
        ) {
            cls.ensure_init().expect("init failed");

            let mut vec = vec![];
            cls.iter_static_fields(|field, val| {
                vec.push((field.name.clone(), field.desc.clone(), val));
                SuperIteration::KeepGoing
            });

            let expected = expected
                .into_iter()
                .map(|(name, ty, val)| {
                    (
                        NativeString::from_utf8(name.as_bytes()),
                        DataType::from_descriptor(mstr::from_literal(ty)).expect("bad descriptor"),
                        val,
                    )
                })
                .collect_vec();
            assert_eq!(vec, expected)
        }

        check_statics(
            &inner1,
            vec![
                ("SIX", "I", DataValue::Int(6)),
                ("FIVE", "I", DataValue::Int(5)),
            ],
        );

        let outer = get_class("Inheritance");
        check_statics(&outer, vec![("FIVE", "I", DataValue::Int(5))]);

        check_statics(
            &inner2,
            vec![
                ("SIX", "I", DataValue::Int(6)),
                ("FIVE", "I", DataValue::Int(5)),
            ],
        );
    }

    #[test]
    fn instance_field_layout_inheritance() {
        test_logging();
        let _jvm = test_jvm();

        let outer = get_class("Inheritance");
        let inner1 = get_class("Inheritance$Inner");
        let inner2 = get_class("Inheritance$InnerDeeper");

        fn get_field_id(cls: &VmRef<Class>, name: &'static str) -> FieldId {
            cls.find_instance_field_recursive(
                mstr::from_literal(name),
                &DataType::from_descriptor(mstr::from_literal("I")).expect("bad descriptor"),
            )
            .expect("field not found")
        }

        assert_eq!(get_field_id(&outer, "five").get(), 0);

        assert_eq!(get_field_id(&inner1, "five").get(), 1);
        assert_eq!(get_field_id(&inner1, "six").get(), 0);

        assert_eq!(get_field_id(&inner2, "five").get(), 1);
        assert_eq!(get_field_id(&inner2, "six").get(), 0);
    }

    #[test]
    fn mangling_method_names() {
        let mangled = MangledMethodName::new(
            mstr::from_literal("my/package/Cool"),
            mstr::from_literal("doThings_lol"),
        );

        fn cstring(bytes: &[u8]) -> &CStr {
            CStr::from_bytes_with_nul(bytes).unwrap()
        }

        assert_eq!(
            mangled.as_ref(),
            cstring(b"Java_my_package_Cool_doThings_1lol\0")
        );

        //                                      look out  --v
        let sig = mstr::from_literal("(ILjava/lang/Objêct;[J)D");
        assert!(MethodSignature::is_valid(sig));
        let long = mangled.into_long(sig);

        assert_eq!(
            long.as_ref(),
            cstring(b"Java_my_package_Cool_doThings_1lol__ILjava_lang_Obj_000eact_2_3J\0")
        )
    }

    #[test]
    fn wtf() {
        let super_class: Option<VmRef<Class>> = None;
        let name: InternedString = MString::from_utf8(b"java/lang/Object");

        if super_class.is_none() != (name.as_bytes() == b"java/lang/Object") {
            panic!(
                "super class is required or otherwise must be Object (super={:?}, name={:?})",
                super_class, name
            );
        }
    }
}
