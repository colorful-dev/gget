mod gget;
use gget::GGet;

fn main() -> anyhow::Result<()> {
    let gget = GGet::run()?;
    println!("{:?}", gget);
    Ok(())
}
