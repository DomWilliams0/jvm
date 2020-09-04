use crate::classpath::ClassPath;
use crate::properties::SystemProperties;
use crate::JvmResult;
use itertools::Itertools;
use log::*;
use std::path::PathBuf;
use thiserror::*;

pub struct Jvm {
    // loader: ClassLoader,
    main: String,
}

#[derive(Default, Debug)]
pub struct JvmArgs {
    pub properties: SystemProperties,
    pub main: String,
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Unknown argument: _0")]
    Unknown(String),

    #[error("Missing main class")]
    MissingMain,

    #[error("Missing boot classpath")]
    MissingBoot,
}

impl Jvm {
    pub fn new(args: JvmArgs) -> JvmResult<Self> {
        // TODO load java/lang/* with native bootstrap classloader

        // TODO set all properties in gnu/classpath/VMSystemProperties.preinit

        Ok(Self { main: args.main })
    }

    pub fn run_main(&mut self) -> JvmResult<()> {
        // // TODO this is a playground for now
        // let path = self.loader.find(&self.main).expect("bad main");
        // info!("found main at {:?}", path);
        //
        // let bytes = std::fs::read(path).expect("io");
        // let class = javaclass::load_from_buffer(&bytes).expect("bad class");
        // info!("class: {:?}", class);

        Ok(())
    }
}

impl JvmArgs {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, ArgError> {
        let mut jvm_args = Self::default();

        // TODO actually parse args with something like clap
        jvm_args.main = args.next().ok_or(ArgError::MissingMain)?;

        let bootclasspath = ClassPath::new(vec![args.next().ok_or(ArgError::MissingBoot)?.into()]);
        let classpath = ClassPath::new(args.map(PathBuf::from).collect_vec());

        jvm_args.properties.set_path("java.class.path", &classpath);
        jvm_args
            .properties
            .set_path("sun.boot.class.path", &bootclasspath);

        Ok(jvm_args)
    }
}
