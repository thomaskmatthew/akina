mod pokemon;
mod save;
mod constrant;

fn main() -> std::io::Result<()> {
    save::save()?;

    Ok(())
}
