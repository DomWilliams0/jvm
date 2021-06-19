use std::error::Error;

use env_logger::Env;
use jvm::{Jvm, JvmArgs};
use log::*;

#[cfg(not(feature = "miri"))]
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::new().filter_or("JVM_LOG", "DEBUG"));

    let args = JvmArgs::parse(std::env::args().skip(1))?;
    debug!("args: {:#?}", args);

    let mut jvm = Jvm::new(args)?;
    jvm.run_main()?;

    Ok(())
}

#[cfg(feature = "miri")]
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(LevelFilter::Trace)
        .init();

    let args = JvmArgs::miri_in_memory();
    let mut jvm = Jvm::new(args)?;
    jvm.run_main()?;

    Ok(())
}
