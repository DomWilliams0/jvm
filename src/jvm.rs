use std::sync::Arc;

use log::*;

use clap::{App, AppSettings, Arg};
use thiserror::*;

use crate::class::null;
use crate::classloader::{ClassLoader, WhichLoader};
use crate::classpath::ClassPath;
use crate::error::ResultExt;
use crate::interpreter::{Frame, InstructionLookupTable, InterpreterResult};
use crate::jit::{JitClient, JitThread};
use crate::properties::SystemProperties;
use crate::thread::JvmThreadState;
use crate::types::DataValue;
use crate::{thread, JvmError, JvmResult};
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;
use std::iter::once;

pub struct Jvm {
    args: JvmArgsPersist,
    state: Arc<JvmGlobalState>,
    jit: JitThread,
}

/// Each thread shares a reference through an Arc
pub struct JvmGlobalState {
    classloader: ClassLoader,
    insn_lookup: InstructionLookupTable,
    jit: JitClient,
    properties: SystemProperties,
}

#[derive(Default, Debug)]
struct JvmArgsPersist {
    main: String,
    no_system_classloader: bool,
}

#[derive(Default, Debug)]
pub struct JvmArgs {
    properties: SystemProperties,

    bootclasspath: Arc<ClassPath>,
    userclasspath: Arc<ClassPath>,

    args: JvmArgsPersist,
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
        let (jit, jit_client) = JitThread::start();

        // create global JVM state
        let global = Arc::new(JvmGlobalState {
            classloader,
            insn_lookup: InstructionLookupTable::new(),
            jit: jit_client,
            properties: args.properties,
        });

        let jvm = Jvm {
            args: args.args,
            state: global.clone(),
            jit,
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
        let loader = if self.args.no_system_classloader {
            WhichLoader::Bootstrap
        } else {
            let system_loader = class_loader.system_classloader().throw()?;
            WhichLoader::User(system_loader)
        };

        // load main class
        let main_class = class_loader
            .load_class(&self.args.main.to_mstr(), loader)
            .throw()?;

        // find main method
        let main_method = main_class
            .find_callable_method(
                "main".as_mstr(),
                "([Ljava/lang/String;)V".as_mstr(),
                MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            )
            .throw()?;

        // TODO populate String[] args
        let args_array = null();

        // execute it!!
        let interp = thread.interpreter();
        let frame =
            Frame::new_with_args(main_method, once(DataValue::Reference(args_array))).unwrap();

        interp.state_mut().push_frame(frame);
        if let InterpreterResult::Exception = interp.execute_until_return() {
            let exc = thread.exception().unwrap();
            Err(JvmError::ExceptionThrown(exc))
        } else {
            Ok(())
        }
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
            .arg(Arg::with_name("nosystemclassloader").long("XXnosystemclassloader"))
            .get_matches_from(args);

        let mut jvm_args = Self::default();

        jvm_args.args.main = matches
            .value_of("class")
            .ok_or(ArgError::MissingMain)?
            .to_owned();
        jvm_args.args.no_system_classloader = matches.is_present("nosystemclassloader");

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

    pub(crate) fn jit(&self) -> &JitClient {
        &self.jit
    }

    pub(crate) fn properties(&self) -> &SystemProperties {
        &self.properties
    }
}
