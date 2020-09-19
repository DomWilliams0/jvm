use std::path::PathBuf;
use std::sync::Arc;

use itertools::Itertools;
use log::*;

use clap::{App, AppSettings, Arg};
use thiserror::*;

use crate::classloader::ClassLoader;
use crate::classpath::ClassPath;
use crate::error::ResultExt;
use crate::interpreter::InstructionLookupTable;
use crate::properties::SystemProperties;
use crate::thread::JvmThreadState;
use crate::{thread, JvmResult};

pub struct Jvm {
    main: String,
    state: Arc<JvmGlobalState>,
}

/// Each thread shares a reference through an Arc
pub struct JvmGlobalState {
    classloader: ClassLoader,
    insn_lookup: InstructionLookupTable,
}

#[derive(Default, Debug)]
pub struct JvmArgs {
    properties: SystemProperties,
    main: String,

    bootclasspath: Arc<ClassPath>,
    userclasspath: Arc<ClassPath>,
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Unknown argument: {0}")]
    Unknown(String),

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
            classloader,
            insn_lookup: InstructionLookupTable::new(),
        });

        let jvm = Jvm {
            main: args.main,
            state: global.clone(),
        };

        // initialise main thread TLS
        thread::initialise(Arc::new(JvmThreadState::new(global)));

        // load system classes
        if let Err(e) = jvm.state.classloader.init_bootstrap_classes().throw() {
            error!("failed to initialise bootstrap classes: {}", e);
            return Err(e);
        }

        // TODO set all properties in gnu/classpath/VMSystemProperties.preinit

        Ok(jvm)
    }

    pub fn run_main(&mut self) -> JvmResult<()> {
        let thread = thread::get();
        let class_loader = thread.global().class_loader();

        // instantiate system classloader
        let system_loader = class_loader.system_classloader().throw()?;

        // TODO load main class with system loader
        panic!("good job getting this far")
    }

    pub fn destroy(&mut self) -> JvmResult<()> {
        // TODO wait for threads to die, unintialise TLS, assert this is the last ref to global state
        todo!()
    }
}

impl JvmArgs {
    pub fn parse(args: impl Iterator<Item = String>) -> Result<Self, ArgError> {
        // TODO standard jvm args
        let matches = App::new("JVM")
            .global_settings(&[AppSettings::NoBinaryName])
            .arg(Arg::with_name("class").help("Class of which to execute main method"))
            .arg(Arg::with_name("cp").long("cp").takes_value(true))
            .arg(
                Arg::with_name("bootcp")
                    .long("Xbootclasspath")
                    .takes_value(true),
            )
            .get_matches_from(args);

        let mut jvm_args = Self::default();

        jvm_args.main = matches
            .value_of("class")
            .ok_or(ArgError::MissingMain)?
            .to_owned();

        let bootclasspath =
            ClassPath::from_colon_separated(matches.value_of("bootcp").unwrap_or(""));
        let classpath = ClassPath::from_colon_separated(matches.value_of("cp").unwrap_or(""));

        // setup properties
        jvm_args.properties.set_path("java.class.path", &classpath);
        jvm_args
            .properties
            .set_path("sun.boot.class.path", &bootclasspath);

        jvm_args.bootclasspath = Arc::new(bootclasspath);
        jvm_args.userclasspath = Arc::new(classpath);

        Ok(jvm_args)
    }
}

impl JvmGlobalState {
    pub(crate) fn class_loader(&self) -> &ClassLoader {
        &self.classloader
    }

    pub(crate) fn insn_lookup(&self) -> &InstructionLookupTable {
        &self.insn_lookup
    }
}
