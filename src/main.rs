fn main() -> anyhow::Result<()> {
    simple_logger::init()?;
    log::info!("Hello, World");
    Ok(())
}
