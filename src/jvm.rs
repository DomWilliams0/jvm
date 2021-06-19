use std::iter::once;
use std::sync::Arc;

use clap::{App, AppSettings, Arg};
use log::*;
use thiserror::*;

use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;

use crate::bootstrap;
use crate::class::null;
use crate::class::{ClassLoader, WhichLoader};
use crate::classpath::ClassPath;
use crate::error::ResultExt;
use crate::interpreter::{Frame, InstructionLookupTable};
use crate::jit::{JitClient, JitThread};
use crate::jni::NativeLibraries;
use crate::properties::SystemProperties;
use crate::thread::JvmThreadState;
use crate::types::DataValue;
use crate::{thread, JvmError, JvmResult};
use parking_lot::Mutex;

use std::ops::DerefMut;

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
    native_libraries: Mutex<NativeLibraries>,
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
            native_libraries: Mutex::new(NativeLibraries::default()),
        });

        let jvm = Jvm {
            args: args.args,
            state: global.clone(),
            jit,
        };

        // initialise main thread TLS
        thread::initialise(Arc::new(JvmThreadState::new(global)));

        // load system classes
        if let Err(e) = bootstrap::init_bootstrap_classes(&jvm.state.classloader).throw() {
            error!("failed to initialise bootstrap classes: {}", e);
            return Err(e);
        }

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

        interp
            .execute_frame(frame)
            .map_err(JvmError::ExceptionThrown)
            .map(|_| /* swallow return value */ ())
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
            // TODO generic -D arg collection
            .arg(
                Arg::with_name("librarypath")
                    .long("XXlibrarypath")
                    .takes_value(true),
            )
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
        jvm_args.properties.set_path(
            "java.library.path",
            &ClassPath::from_colon_separated(matches.value_of("librarypath").unwrap_or(".")),
        );

        jvm_args.bootclasspath = Arc::new(bootclasspath);
        jvm_args.userclasspath = Arc::new(classpath);

        Ok(jvm_args)
    }

    #[cfg(feature = "miri")]
    pub fn miri_in_memory() -> Self {
        Self {
            properties: SystemProperties::new(
                "",
                "",
                "miri".to_string(),
                "miri".to_string(),
                "",
                "",
                "lib/classpath",
            ),
            bootclasspath: Arc::new(ClassPath::from_colon_separated("share/classpath")),
            userclasspath: Arc::new(ClassPath::default()),
            args: JvmArgsPersist {
                main: "Nop".to_string(),
                no_system_classloader: false,
            },
        }
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

    pub(crate) fn native_libraries_mut(&self) -> impl DerefMut<Target = NativeLibraries> + '_ {
        self.native_libraries.lock()
    }
}
