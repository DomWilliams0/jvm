use std::error::Error;

#[cfg(feature = "find_native_bin")]
mod nice {
    use glob::glob;
    use itertools::Itertools;
    use std::error::Error;
    use std::ffi::OsStr;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use mutf8::mstr;

    #[derive(Default)]
    struct Collect {
        methods: Vec<NativeMethod>,
    }

    #[derive(Debug)]
    struct NativeMethod {
        /// java/lang/Lala
        java_cls: String,
        name: String,
        desc: String,
    }

    fn cls_to_rustfile(cls: &str) -> String {
        cls.to_lowercase().replace('/', "_")
    }

    fn rust_method_name(cls: &str, method: &str) -> String {
        let rust = cls_to_rustfile(cls);
        let mut final_cls = cls
            .split('/')
            .last()
            .expect("missing last split")
            .to_lowercase();
        if final_cls.starts_with("vm") {
            final_cls.insert(2, '_');
        }
        let method = method.to_lowercase();
        format!("{rust}::{final_cls}_{method}")
    }

    impl Collect {
        fn parse(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
            println!("parsing {}", path.display());
            let buf = std::fs::read(path)?;
            let classfile = cafebabe::load_from_buffer(&buf)?;

            let cls_name = classfile.this_class()?.to_str();
            if cls_name.starts_with("gnu/java") || cls_name.starts_with("gnu/xml") {
                // skip for nw
                return Ok(());
            }

            for method in classfile.methods() {
                if method.access_flags.is_native() {
                    let native = NativeMethod {
                        java_cls: classfile.this_class()?.to_string(),
                        name: method.name.to_string(),
                        desc: method.descriptor.to_string(),
                    };
                    println!("{:?}", native);

                    self.methods.push(native);
                }
            }

            Ok(())
        }
    }

    pub fn main() -> Result<(), Box<dyn Error>> {
        let root = PathBuf::from(std::env::var("JVM_CLASSPATH_DIR").expect("missing env var"));
        let mut collect = Collect::default();

        for class in glob(&format!("{}/**/*.class", root.display()))? {
            let e = class?;
            if e.extension() == Some(OsStr::new("class")) {
                match collect.parse(&e) {
                    Err(e) => {
                        println!("failed to parse: {}", e);
                        continue;
                    }
                    Ok(x) => x,
                }
            }
        }

        let out = PathBuf::from(format!("./generated-native"));
        let _ = std::fs::remove_dir_all(&out);
        let _ = std::fs::create_dir(&out);

        // mod
        {
            let modrs = out.join("mod.rs");
            let mut f = File::create(modrs)?;
            let classes = collect
                .methods
                .iter()
                .map(|m| &m.java_cls)
                .dedup()
                .cloned()
                .sorted()
                .collect_vec();

            for cls in classes {
                write!(&mut f, "mod {};\n", cls_to_rustfile(&cls))?;
            }
        }

        // preload
        {
            //          Preload::with_natives(
            //          "gnu/classpath/VMSystemProperties",
            //          &[(
            //          "preInit",
            //          "(Ljava/util/Properties;)V",
            //          gnu_classpath_vmsystemproperties::vm_systemproperties_preinit,
            //          )],
            //          ),
            let mut f = File::create(out.join("preload.txt"))?;
            for (cls, methods) in &collect
                .methods
                .iter()
                .sorted_by_key(|m| &m.java_cls)
                .group_by(|m| &m.java_cls)
            {
                write!(
                    &mut f,
                    r#"
                     Preload::with_natives(
                     "{}",
                     &[
                     "#,
                    cls
                )?;

                for m in methods {
                    let rust = rust_method_name(&cls, &m.name);
                    write!(
                        &mut f,
                        r#"
                        ("{}", "{}", {}),
                "#,
                        m.name, m.desc, rust
                    )?;
                }

                write!(&mut f, "],")?;
            }
        }

        // rust stubs
        {
            for (cls, methods) in &collect
                .methods
                .iter()
                .sorted_by_key(|m| &m.java_cls)
                .group_by(|m| &m.java_cls)
            {
                let mut buf = out.join("rust").join(cls_to_rustfile(&cls));
                buf.set_extension("rs");
                std::fs::create_dir_all(buf.parent().unwrap())?;
                let mut f = File::create(buf)?;
                write!(
                    &mut f,
                    r#"
use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;
"#,
                )?;

                for m in methods {
                    let fq_name = rust_method_name(&cls, &m.name);
                    let name = fq_name.split("::").nth(1).unwrap();

                    write!(
                        &mut f,
                        r#"
pub fn {}(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {{
    todo!("native method {}");
}}

                "#,
                        name, fq_name
                    )?;
                }
            }
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cls_name() {
            assert_eq!(
                cls_to_rustfile("java/reflect/Constructor"),
                "java_reflect_constructor"
            );
            assert_eq!(
                cls_to_rustfile("gnu/classpath/VMSystemProperties"),
                "gnu_classpath_vmsystemproperties"
            );
        }

        #[test]
        fn method_name() {
            assert_eq!(
                rust_method_name("gnu/classpath/VMSystemProperties", "preInit"),
                "gnu_classpath_vmsystemproperties::vm_systemproperties_preinit"
            )
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "find_native_bin")]
    {
        nice::main()
    }
    #[cfg(not(feature = "find_native_bin"))]
    {
        panic!("needs find_native_bin feature")
    }
}
