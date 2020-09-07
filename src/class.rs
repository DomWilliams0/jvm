use std::collections::HashMap;
use std::mem::MaybeUninit;

use cafebabe::{attribute, AccessFlags, ClassError, FieldAccessFlags, MethodAccessFlags};
use lazy_static::lazy_static;
use log::*;

use crate::alloc::{vmref_ptr, InternedString, NativeString, VmRef};
use crate::classloader::{ClassLoader, WhichLoader};
use crate::error::{Throwables, VmResult};
use crate::types::{DataType, DataValue};
use cafebabe::mutf8::mstr;

use crate::constant_pool::RuntimeConstantPool;
use cafebabe::attribute::Code;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Class {
    name: InternedString,
    source_file: Option<NativeString>,

    /// java/lang/Class instance
    /// TODO weak reference for cyclic?
    class_object: MaybeUninit<VmRef<Object>>,

    /// Only None for java/lang/Object
    super_class: Option<VmRef<Class>>,

    interfaces: Vec<VmRef<Class>>,
    fields: Vec<Field>,
    methods: Vec<VmRef<Method>>,

    constant_pool: RuntimeConstantPool,

    // name -> value. disgusting
    static_field_values: HashMap<NativeString, DataValue>,
}

pub struct Object {
    class: VmRef<Class>,
}

lazy_static! {
    pub static ref NULL: VmRef<Object> = {
        // TODO just allocate an object instead of this unsafeness
        let null_class = MaybeUninit::zeroed();
        let null_class = unsafe { null_class.assume_init() };
        VmRef::new(Object { class: null_class })
    };
}

#[derive(Debug)]
pub struct Field {
    name: NativeString,
    desc: NativeString,
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

impl Class {
    pub fn link(
        expected_name: &mstr,
        loaded: cafebabe::ClassFile,
        loader: WhichLoader,
        classloader: &mut ClassLoader,
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

        let fields: Vec<_> = loaded
            .fields()
            .map(|f| Field {
                name: f.name.to_owned(),
                desc: f.descriptor.to_owned(),
                flags: f.access_flags,
            })
            .collect();

        // preparation step - initialise static fields
        // TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors
        let static_field_values = {
            let mut map =
                HashMap::with_capacity(fields.iter().filter(|f| f.flags.is_static()).count());
            for field in &fields {
                if field.flags.is_static() {
                    let value = match DataType::from_descriptor(&field.desc) {
                        Some(dt) => {
                            trace!("static field {:?} has type {:?}", field.name, dt);
                            dt.default_value()
                        }
                        None => {
                            warn!("unknown type descriptor {:?}", field.desc);
                            return Err(Throwables::ClassFormatError);
                        }
                    };

                    map.insert(field.name.clone(), value);
                }
            }

            map
        };

        let constant_pool =
            RuntimeConstantPool::from_cafebabe(loaded.constant_pool()).map_err(|e| {
                warn!("invalid constant pool: {}", e);
                Throwables::ClassFormatError
            })?;

        // alloc self with uninitialised object ptr
        let vm_class = VmRef::new(Self {
            name,
            source_file,
            class_object: MaybeUninit::zeroed(),
            super_class,
            interfaces,
            methods,
            constant_pool,
            static_field_values,
            fields,
        });

        // alloc java/lang/Class
        let obj = VmRef::new(Object {
            class: vm_class.clone(),
        });

        // update ptr - im sure at some point we will need to mutate the class so this *const to
        // *mut hacky is temporary until that's needed
        unsafe {
            let ptr = vm_class.class_object.as_ptr();
            let ptr = ptr as *mut VmRef<Object>;
            ptr.write(obj);
        }

        // TODO set obj->vmdata field to vm_class

        Ok(vm_class)
    }

    fn find_method(
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

    pub fn find_static_constructor(&self) -> MethodLookupResult {
        self.find_method(
            mstr::from_utf8(b"<clinit>").as_ref(),
            mstr::from_mutf8(b"()V").as_ref(),
            MethodAccessFlags::STATIC,
        )
    }

    pub fn name(&self) -> &mstr {
        &self.name
    }

    pub const fn constant_pool(&self) -> &RuntimeConstantPool {
        &self.constant_pool
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
