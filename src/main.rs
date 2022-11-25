mod gget;
use gget::GGet;

mod config;
mod request;
mod utils;

fn main() -> anyhow::Result<()> {
    let gget = GGet::run()?;
    println!("{:?}", gget);
    Ok(())
}
