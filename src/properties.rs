use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::alloc::vmref_alloc_object;
use crate::class::{FunctionArgs, Object};
use crate::classpath::ClassPath;
use crate::interpreter::{Frame, InterpreterResult};
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;
use cafebabe::MethodAccessFlags;

#[derive(Debug)]
pub struct SystemProperties(HashMap<&'static str, SystemProperty>);

#[derive(Debug)]
pub struct SystemProperty(Cow<'static, str>);

impl Default for SystemProperties {
    fn default() -> Self {
        let defaults = [
            ("java.vm.name", "UntitledJvm"),
            ("java.vm.vendor", "Dom Williams"),
            // TODO remaining static ones
            // TODO dynamic ones e.g. user.home
        ];

        SystemProperties(HashMap::from_iter(
            defaults
                .iter()
                .map(|(key, val)| (*key, SystemProperty::from(*val))),
        ))
    }
}

impl SystemProperties {
    pub fn get(&self, key: &str) -> Option<&SystemProperty> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: &'static str, value: impl Into<SystemProperty>) {
        self.0.insert(key, value.into());
    }

    pub fn set_path(&mut self, key: &'static str, value: &ClassPath) {
        self.set(key, value.to_string())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.0.iter().map(|(key, val)| (*key, val.0.as_ref()))
    }
}

impl From<&'static str> for SystemProperty {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl From<String> for SystemProperty {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

// required:
// java.version
// java.vendor
// java.vendor.url
// java.home
// java.vm.specification.version
// java.vm.specification.vendor
// java.vm.specification.name
// java.vm.version
// java.vm.vendor
// java.vm.name
// java.specification.version
// java.specification.vendor
// java.specification.name
// java.class.version
// java.class.path
// java.library.path
// java.io.tmpdir
// java.compiler
// java.ext.dirs
// os.name
// os.arch
// os.version
// file.separator
// path.separator
// line.separator
// user.name
// user.home
// user.dir

pub fn vm_systemproperties_preinit(mut args: FunctionArgs) -> Option<DataValue> {
    let props = args.take(0).into_reference().unwrap();
    // TODO actually do preInit

    let thread = thread::get();
    let system_properties = thread.global().properties();
    let interpreter = thread.interpreter();

    // lookup method once
    let props_class = props.class().unwrap();
    let method = props_class
        .find_callable_method(
            mstr::from_mutf8(b"setProperty"),
            mstr::from_mutf8(b"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;"),
            MethodAccessFlags::empty(),
        )
        .expect("cant find setProperty");

    for (key, val) in system_properties.iter() {
        log::debug!("setting property {:?} => {:?}", key, val);

        // alloc jvm string
        let key =
            vmref_alloc_object(|| Object::new_string(mstr::from_utf8(key.as_bytes()).as_ref()))
                .expect("bad key");
        let val =
            vmref_alloc_object(|| Object::new_string(mstr::from_utf8(val.as_bytes()).as_ref()))
                .expect("bad value");

        // make frame for method call
        let args = [props.clone(), key, val];
        let frame = Frame::new_with_args(
            method.clone(),
            args.iter().map(|o| DataValue::Reference(o.to_owned())),
        )
        .expect("can't make frame");

        interpreter.state_mut().push_frame(frame);
        let ret = interpreter.execute_until_return();
        // TODO use ret
        assert!(matches!(ret, InterpreterResult::Success))
    }

    None // void
}
