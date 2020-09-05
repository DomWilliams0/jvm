use std::path::PathBuf;
use std::sync::Arc;

use itertools::Itertools;
use log::*;
use parking_lot::RwLock;
use thiserror::*;

use crate::alloc::NativeString;
use crate::classloader::ClassLoader;
use crate::classpath::ClassPath;
use crate::error::ResultExt;
use crate::properties::SystemProperties;
use crate::thread::JvmThreadState;
use crate::{thread, JvmResult};

pub struct Jvm {
    main: NativeString,
    state: Arc<JvmGlobalState>,
}

/// Each thread shares a reference through an Arc
pub struct JvmGlobalState {
    classloader: RwLock<ClassLoader>,
}

#[derive(Default, Debug)]
pub struct JvmArgs {
    properties: SystemProperties,
    main: NativeString,

    bootclasspath: Arc<ClassPath>,
    userclasspath: Arc<ClassPath>,
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Unknown argument: {0}")]
    Unknown(NativeString),

    #[error("Missing main class")]
    MissingMain,

    #[error("Missing boot classpath")]
    MissingBoot,
}

impl Jvm {
    // TODO "catch" any exception during init, and log it properly with stacktrace etc
    pub fn new(args: JvmArgs) -> JvmResult<Self> {
        let classloader = ClassLoader::new(args.bootclasspath.clone());

        // create global JVM state
        let global = Arc::new(JvmGlobalState {
            classloader: RwLock::new(classloader),
        });

        let jvm = Jvm {
            main: args.main,
            state: global.clone(),
        };

        // initialise main thread TLS
        thread::initialise(Arc::new(JvmThreadState::new(global)));

        // load system classes
        {
            let mut cl = jvm.state.classloader.write();
            if let Err(e) = cl.init_bootstrap_classes().throw() {
                error!("failed to initialise bootstrap classes: {}", e);
                return Err(e);
            }
        }

        // TODO set all properties in gnu/classpath/VMSystemProperties.preinit

        Ok(jvm)
    }

    pub fn run_main(&mut self) -> JvmResult<()> {
        panic!("good job getting this far")
    }

    pub fn destroy(&mut self) -> JvmResult<()> {
        // TODO wait for threads to die, unintialise TLS, assert this is the last ref to global state
        todo!()
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

        jvm_args.bootclasspath = Arc::new(bootclasspath);
        jvm_args.userclasspath = Arc::new(classpath);

        Ok(jvm_args)
    }
}
