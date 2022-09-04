use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{Class, FieldSearchType, Method, Object};
use crate::error::{Throwables, VmResult};
use crate::interpreter::Frame;
use crate::thread::JvmThreadState;
use crate::types::DataValue;
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;

/// Helper fns for invoking methods and setting fields from vm
pub struct ExecHelper<'a> {
    state: &'a JvmThreadState,
}

// TODO class arg should be a trait for either class name &str or class reference

impl<'a> ExecHelper<'a> {
    pub fn new(state: &'a JvmThreadState) -> Self {
        ExecHelper { state }
    }

    /// Bootstrap class
    pub fn instantiate(&self, cls_name: &'static str) -> VmResult<(VmRef<Object>, VmRef<Class>)> {
        let cls = self
            .state
            .global()
            .class_loader()
            .get_bootstrap_class(cls_name);
        vmref_alloc_object(|| Ok(Object::new(cls.clone()))).map(|o| (o, cls))
    }

    /// Boostrap class. Args will have `this` pushed as first. Args should be in decl order
    pub fn instantiate_and_invoke_constructor(
        &self,
        cls: &'static str,
        constructor_desc: &'static str,
        args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> VmResult<VmRef<Object>> {
        let (obj, cls) = self.instantiate(cls)?;

        let constructor = cls
            .find_instance_constructor(constructor_desc.as_mstr())
            .ok_or(Throwables::Other("java/lang/NoSuchMethodError"))?;

        self.invoke_method(constructor, Some(obj.clone()), args)?;
        Ok(obj)
    }

    pub fn invoke_instance_method(
        &self,
        obj: impl Into<DataValue>,
        cls: VmRef<Class>,
        method_name: &'static str,
        method_desc: &'static str,
        args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> VmResult<Option<DataValue>> {
        let method = cls.find_callable_method(
            method_name.as_mstr(),
            method_desc.as_mstr(),
            MethodAccessFlags::empty(),
        )?;

        self.invoke_method(method, Some(obj), args)
    }

    pub fn invoke_static_method(
        &self,
        cls_name: &'static str,
        method_name: &'static str,
        method_desc: &'static str,
        args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> VmResult<Option<DataValue>> {
        let cls = self
            .state
            .global()
            .class_loader()
            .get_bootstrap_class(cls_name);
        let method = cls.find_callable_method(
            method_name.as_mstr(),
            method_desc.as_mstr(),
            MethodAccessFlags::STATIC,
        )?;

        self.invoke_method(method, Option::<DataValue>::None, args)
    }

    /// Args should be in decl order
    fn invoke_method(
        &self,
        method: VmRef<Method>,
        this: Option<impl Into<DataValue>>,
        args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> VmResult<Option<DataValue>> {
        let interp = self.state.interpreter();

        let frame =
            Frame::new_with_args(method, args.rev().chain(this.map(Into::into).into_iter()))
                .unwrap(); // TODO handle exc

        interp.execute_frame(frame).map_err(|exc| {
            let exc_name = exc.class_name;
            self.state.set_exception(exc);
            Throwables::Other(exc_name) // unsure this is fine
        })
    }

    /// Only looks in the given class, not superclass. Cannot be array
    pub fn set_instance_field(
        &self,
        obj: &Object,
        field: &'static str,
        value: DataValue,
    ) -> VmResult<()> {
        let name = field.as_mstr();
        let datatype = value.data_type();
        let field_id = obj
            .find_field_in_this_only(name.as_ref(), &datatype, FieldSearchType::Instance)
            .ok_or(Throwables::Other("java/lang/NoSuchFieldError"))?;

        obj.fields().unwrap().ensure_set(field_id, value);
        Ok(())
    }
}
