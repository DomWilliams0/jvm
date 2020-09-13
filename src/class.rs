use std::mem::MaybeUninit;

use cafebabe::{
    attribute, AccessFlags, ClassAccessFlags, ClassError, FieldAccessFlags, MethodAccessFlags,
};
use lazy_static::lazy_static;
use log::*;

use crate::alloc::{vmref_ptr, InternedString, NativeString, VmRef};
use crate::classloader::{current_thread, ClassLoader, WhichLoader};
use crate::error::{Throwables, VmResult};
use crate::types::{DataType, DataValue, PrimitiveDataType};
use cafebabe::mutf8::mstr;

use crate::constant_pool::RuntimeConstantPool;
use crate::monitor::{Monitor, MonitorGuard};
use crate::storage::{FieldMapStorage, Storage};
use crate::thread;
use cafebabe::attribute::Code;
use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::thread::ThreadId;

#[derive(Debug)]
pub enum ClassType {
    Array(VmRef<Class>),
    Primitive(PrimitiveDataType),
    Normal,
}

#[derive(Debug)]
pub struct Class {
    name: InternedString,
    class_type: ClassType,
    source_file: Option<NativeString>,
    state: LockedClassState,
    loader: WhichLoader,

    access_flags: ClassAccessFlags,

    /// java/lang/Class instance
    /// TODO weak reference for cyclic?
    class_object: MaybeUninit<VmRef<Object>>,

    /// Only None for java/lang/Object
    super_class: Option<VmRef<Class>>,

    interfaces: Vec<VmRef<Class>>,
    fields: Vec<Field>,
    methods: Vec<VmRef<Method>>,

    constant_pool: RuntimeConstantPool,

    static_field_values: FieldMapStorage,
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

pub struct Object {
    class: VmRef<Class>,
    monitor: Monitor,
    fields: FieldMapStorage,
}

lazy_static! {
    pub static ref NULL: VmRef<Object> = VmRef::new(Object::new_null());
}

#[derive(Debug)]
pub struct Field {
    name: NativeString,
    desc: DataType,
    flags: FieldAccessFlags,
}

#[derive(Debug)]
pub struct Method {
    name: NativeString,
    desc: NativeString,
    flags: MethodAccessFlags,

    /// Only present if not native or abstract
    code: Option<attribute::Code>,
    attributes: Vec<attribute::OwnedAttribute>,
}

pub enum MethodLookupResult {
    Found(VmRef<Method>),
    FoundMultiple,
    NotFound,
}

// TODO get classloader reference from tls instead of parameter

impl Class {
    pub fn link(
        expected_name: &mstr,
        loaded: cafebabe::ClassFile,
        loader: WhichLoader,
        classloader: &ClassLoader,
    ) -> VmResult<VmRef<Self>> {
        debug!("linking class {:?}", expected_name);

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
                let super_class = classloader.load_class(super_name, loader.clone())?;
                Some(super_class)
            }
            Err(ClassError::NoSuper) if name.as_bytes() == b"java/lang/Object" => {
                // the one exception, no super class expected
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

                let interface = classloader.load_class(interface_name, loader.clone())?;
                vec.push(interface);
            }
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
                    if let Some(idx) = idx {
                        if method
                            .access_flags
                            .intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE)
                        {
                            warn!(
                                "abstract or native method {:?} has Code attribute",
                                method.name
                            );
                            return Err(Throwables::ClassFormatError);
                        }

                        // pop from attributes list
                        let code = match attributes.swap_remove(idx) {
                            attribute::OwnedAttribute::Code(code) => code,
                            _ => unreachable!(),
                        };

                        Some(code)
                    } else {
                        None
                    }
                };

                vec.push(VmRef::new(Method {
                    name: method.name.to_owned(),
                    desc: method.descriptor.to_owned(),
                    flags: method.access_flags,
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
                    desc,
                    flags: field.access_flags,
                })
            }

            vec
        };

        // preparation step - initialise static fields
        // TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors
        let static_field_values = {
            let mut static_fields = FieldMapStorage::with_capacity(
                fields.iter().filter(|f| f.flags.is_static()).count(),
            );
            for field in &fields {
                if field.flags.is_static() {
                    static_fields.put(field.name.clone(), field.desc.clone().default_value());
                }
            }

            static_fields
        };

        let constant_pool =
            RuntimeConstantPool::from_cafebabe(loaded.constant_pool()).map_err(|e| {
                warn!("invalid constant pool: {}", e);
                Throwables::ClassFormatError
            })?;

        let access = loaded.access_flags();

        Ok(Self::new(
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
            static_field_values,
        ))
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
            FieldMapStorage::with_capacity(0),
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
            FieldMapStorage::with_capacity(0),
        );

        Ok(cls)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
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
        static_field_values: FieldMapStorage,
    ) -> VmRef<Class> {
        debug_assert!(super_class.is_none() == (name.as_bytes() == b"java/lang/Object"));

        let vm_class = VmRef::new(Self {
            name,
            class_type,
            access_flags,
            source_file,
            state: LockedClassState::default(),
            loader,
            class_object: MaybeUninit::zeroed(),
            super_class,
            interfaces,
            methods,
            constant_pool,
            static_field_values,
            fields,
        });

        // alloc java/lang/Class
        let obj = VmRef::new(Object::new(vm_class.clone()));

        // update ptr - TODO use Arc::get_unchecked_mut when it is stable
        unsafe {
            let ptr = vm_class.class_object.as_ptr();
            let ptr = ptr as *mut VmRef<Object>;
            ptr.write(obj);
        }

        // TODO set obj->vmdata field to vm_class

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
            mstr::from_utf8(b"<clinit>").as_ref(),
            mstr::from_mutf8(b"()V").as_ref(),
            MethodAccessFlags::STATIC,
        )
    }

    pub fn find_instance_constructor(&self, descriptor: &mstr) -> Option<VmRef<Method>> {
        debug_assert!(descriptor.to_utf8().ends_with("V"));

        self.find_method(
            mstr::from_utf8(b"<init>").as_ref(),
            descriptor,
            MethodAccessFlags::empty(),
        )
        .and_then(|method| {
            if method.flags.is_static() {
                None
            } else {
                Some(method)
            }
        })
    }

    fn find_method(
        &self,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
    ) -> Option<VmRef<Method>> {
        self.methods
            .iter()
            .find(|m| {
                m.flags.contains(flags) && m.name.as_mstr() == name && m.desc.as_mstr() == desc
            })
            .cloned()
    }

    /// Looks in super classes too
    // TODO version to look in (super)interfaces too
    pub fn find_method_recursive(
        cls: &VmRef<Class>,
        name: &mstr,
        desc: &mstr,
        flags: MethodAccessFlags,
    ) -> Option<VmRef<Method>> {
        let mut current = Some(cls);
        while let Some(cls) = current {
            if let Some(method) = cls.find_method(name, desc, flags) {
                return Some(method);
            }

            current = cls.super_class.as_ref();
        }

        None
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

    fn class_object(&self) -> &VmRef<Object> {
        let ptr = self.class_object.as_ptr();
        // safety: initialised unconditionally in link()
        unsafe { &*ptr }
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

                // recursively initialise super class, interfaces and super interfaces
                let mut result = Ok(());
                self.with_supers(&mut |cls| {
                    trace!("initialising super: {:?}", cls.name);

                    if let Err(e) = cls.ensure_init() {
                        debug!("super class initialisation failed: {:?}", e);
                        result = Err(e);
                        false // stop early
                    } else {
                        true
                    }
                });

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
                            if let Err(e) =
                                interpreter.execute_method(self.clone(), m, None /* static */)
                            {
                                warn!("static constructor failed: {}", e);
                                return Err(Throwables::ClassFormatError); // TODO different exception
                            }
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

    fn with_supers(&self, f: &mut impl FnMut(&VmRef<Class>) -> bool) {
        let mut keep_going = true;

        if let Some(super_class) = self.super_class.as_ref() {
            keep_going = f(super_class);
        }

        let mut ifaces = self.interfaces.iter();
        while keep_going {
            match ifaces.next() {
                Some(iface) => {
                    keep_going = f(iface);

                    if keep_going {
                        iface.with_supers(f);
                    }
                }
                None => break,
            }
        }
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

impl Object {
    /// Only use this to create the sentinel NULL value
    fn new_null() -> Self {
        // TODO just allocate an object instead of this unsafeness
        let null_class = MaybeUninit::zeroed();
        let null_class = unsafe { null_class.assume_init() };
        Object {
            class: null_class,
            monitor: Monitor::new(),
            fields: FieldMapStorage::with_capacity(0),
        }
    }

    pub(crate) fn new(class: VmRef<Class>) -> Self {
        // TODO inherit superclass fields too
        let fields = {
            let mut map = FieldMapStorage::with_capacity(class.fields.len());
            for field in class.fields.iter() {
                map.put(field.name.clone(), field.desc.clone().default_value());
            }

            map
        };
        Object {
            class,
            monitor: Monitor::new(),
            fields,
        }
    }

    pub fn is_null(&self) -> bool {
        VmRef::ptr_eq(&self.class, &NULL.class)
    }

    pub fn class(&self) -> Option<VmRef<Class>> {
        if self.is_null() {
            None
        } else {
            Some(self.class.clone())
        }
    }

    pub fn enter_monitor(&self) -> MonitorGuard {
        self.monitor.enter()
    }

    pub fn get_field_by_name<F: From<DataValue>>(&self, name: &mstr) -> Option<F> {
        self.fields.get(name).map(F::from)
    }

    /// Panics if no field with given name
    pub fn set_field_by_name<F: Into<DataValue>>(&self, name: &mstr, value: F) {
        let value = value.into();
        debug!("setting field {:?} value to {:?}", name, value);
        assert!(self.fields.set(name, value), "no such field {:?}", name);
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else {
            // TODO not quite correct toString
            let ptr = vmref_ptr(&self.class);
            write!(f, "{:?}@{:x}", self.class.name, ptr)
        }
    }
}

impl Method {
    pub fn code(&self) -> Option<&Code> {
        self.code.as_ref()
    }

    pub fn name(&self) -> &mstr {
        &self.name
    }

    pub fn flags(&self) -> MethodAccessFlags {
        self.flags
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
