use jvm::{Jvm, JvmArgs};
use std::error::Error;
use log::*;

fn main() -> Result<(), Box<dyn Error>>{
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let args = JvmArgs::parse(std::env::args().skip(1))?;
    debug!("args: {:#?}", args);

    let mut jvm = Jvm::new(args)?;
    jvm.run_main()?;

    Ok(())
}