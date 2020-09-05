use std::error::Error;

use log::*;

use env_logger::Env;
use jvm::{Jvm, JvmArgs};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::new().filter_or("JVM_LOG", "DEBUG"));

    let args = JvmArgs::parse(std::env::args().skip(1))?;
    debug!("args: {:#?}", args);

    let mut jvm = Jvm::new(args)?;
    jvm.run_main()?;

    Ok(())
}
