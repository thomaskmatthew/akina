

use std::fs;
use std::io;

pub fn save() -> io::Result<()> {
    let original = fs::read("saves/blue.sav")?;
    let vc = fs::read("saves/sav.dat")?;

    println!("Original: {} bytes", original.len());
    println!("VC: {} bytes", vc.len());

    Ok(())
}