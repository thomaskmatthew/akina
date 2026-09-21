
mod save;
mod reader;
mod pokemon;
mod helper;
mod interface;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let files = [
        "saves/blue.sav",
        "saves/sav.dat",
    ];

    for path in files {
        println!("\nReading: {}", path);

        let data = fs::read(path)?;

        println!("File size: {} bytes", data.len());

        reader::print_party(&data)?;
    }

    Ok(())
}