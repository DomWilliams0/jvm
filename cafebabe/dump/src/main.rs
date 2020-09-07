use cafebabe::{load_from_buffer, ClassResult};
use log::LevelFilter;

// really awful test binary
fn main() -> ClassResult<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let bytes = include_bytes!("../../../java/Dummy.class");
    let class = load_from_buffer(bytes)?;
    log::info!("class: {:#?}", class);

    Ok(())
}
