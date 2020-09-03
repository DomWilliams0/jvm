use std::path::PathBuf;
use log::*;
use thiserror::*;
use crate::JvmResult;
use crate::class::ClassLoader;

pub struct Jvm {
    loader: ClassLoader,
    main: String,
}

#[derive(Default, Debug)]
pub struct JvmArgs {
    // TODO bootstrap classpath
    pub classpath: Vec<PathBuf>,
    pub main: String,
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Unknown argument: _0")]
    Unknown(String),

    #[error("Missing main class")]
    MissingMain,
}

impl Jvm {
    pub fn new(args: JvmArgs) -> JvmResult<Self> {
        let loader = ClassLoader::new(args.classpath);
        Ok(Self {loader, main: args.main})
    }

    pub fn run_main(&mut self) -> JvmResult<()> {
        // TODO this is a playground for now
        let path = self.loader.find(&self.main).expect("bad main");
        info!("found main at {:?}", path);

        let bytes = std::fs::read(path).expect("io");
        let class = javaclass::load_from_buffer(&bytes).expect("bad class");
        info!("class: {:?}", class);

        Ok(())
    }
}

impl JvmArgs {
    pub fn parse(mut args: impl Iterator<Item=String>) -> Result<Self, ArgError> {
        let mut jvm_args = Self::default();

        // TODO actually parse args with something like clap
        jvm_args.main = args.next().ok_or(ArgError::MissingMain)?;

        jvm_args.classpath.extend(args.map(PathBuf::from));


        Ok(jvm_args)
    }
}