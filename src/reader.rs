

use std::io::{self, Error, ErrorKind};

// Location of the party in a standard Gen 1 save.
const PARTY_OFFSET: usize = 0x2F2C;

// Each party Pokemon occupies 44 bytes.
const POKEMON_SIZE: usize = 44;

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([
        data[offset],
        data[offset + 1],
    ])
}

fn read_u24(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        0,
        data[offset],
        data[offset + 1],
        data[offset + 2],
    ])
}


pub fn print_party(save: &[u8]) -> io::Result<()> {
    if save.len() != 0x8000 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Expected a 32 KiB Gen 1 save file",
        ));
    }

    let party_count = save[PARTY_OFFSET] as usize;

    if party_count > 6 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid party count",
        ));
    }

    println!("Pokemon in party: {}", party_count);

    for i in 0..party_count {
        let offset = PARTY_OFFSET + 8 + i * POKEMON_SIZE;

        let pokemon = &save[offset..offset + POKEMON_SIZE];

        // Basic information
        let species = pokemon[0x00];
        let level = pokemon[0x21];
        let box_level = pokemon[0x03];

        let name = crate::helper::species_name(species);

        // Health and status
        let current_hp = read_u16(pokemon, 0x01);
        let max_hp = read_u16(pokemon, 0x22);
        let status = pokemon[0x04];

        // Pokemon types
        let type1 = pokemon[0x05];
        let type2 = pokemon[0x06];

        // Catch rate / item-related byte
        let catch_rate = pokemon[0x07];

        // Moves and PP
        let moves = &pokemon[0x08..0x0C];
        let pp = &pokemon[0x1D..0x21];

        // Trainer and experience
        let trainer_id = read_u16(pokemon, 0x0C);
        let experience = read_u24(pokemon, 0x0E);

        // Stat experience
        let hp_exp = read_u16(pokemon, 0x11);
        let attack_exp = read_u16(pokemon, 0x13);
        let defense_exp = read_u16(pokemon, 0x15);
        let speed_exp = read_u16(pokemon, 0x17);
        let special_exp = read_u16(pokemon, 0x19);

        // Individual values (DVs)
        let attack_dv = pokemon[0x1B] >> 4;
        let defense_dv = pokemon[0x1B] & 0x0F;

        let speed_dv = pokemon[0x1C] >> 4;
        let special_dv = pokemon[0x1C] & 0x0F;

        let hp_dv = ((attack_dv & 1) << 3)
            | ((defense_dv & 1) << 2)
            | ((speed_dv & 1) << 1)
            | (special_dv & 1);

        // Battle stats
        let attack = read_u16(pokemon, 0x24);
        let defense = read_u16(pokemon, 0x26);
        let speed = read_u16(pokemon, 0x28);
        let special = read_u16(pokemon, 0x2A);

        // Display all information
        println!("\n=========================");
        println!("Pokemon #{}", i + 1);
        println!("=========================");

        println!("Name: {} (ID: {})", name, species);
        println!("Level: {}", level);
        println!("Box level field: {}", box_level);

        println!("\n--- Health ---");
        println!("HP: {}/{}", current_hp, max_hp);
        println!("Status byte: 0x{:02X}", status);

        println!("\n--- Types ---");
        println!("Type 1 ID: {}", type1);
        println!("Type 2 ID: {}", type2);
        println!("Catch rate/item byte: {}", catch_rate);

        println!("\n--- Battle Stats ---");
        println!("Attack: {}", attack);
        println!("Defense: {}", defense);
        println!("Speed: {}", speed);
        println!("Special: {}", special);

        println!("\n--- Moves ---");


        for slot in 0..4 {
            let move_id = moves[slot];
            let remaining_pp = pp[slot] & 0x3F;
            let pp_ups = pp[slot] >> 6;

            println!(
                "Move {}: {} (ID {}) | PP {} | PP Ups {}",
                slot + 1,
                crate::helper::move_name(move_id),
                move_id,
                remaining_pp,
                pp_ups
            );
        }
        println!("\n--- Experience ---");
        println!("Total EXP: {}", experience);
        println!("HP stat EXP: {}", hp_exp);
        println!("Attack stat EXP: {}", attack_exp);
        println!("Defense stat EXP: {}", defense_exp);
        println!("Speed stat EXP: {}", speed_exp);
        println!("Special stat EXP: {}", special_exp);

        println!("\n--- Individual Values ---");
        println!("HP DV: {}", hp_dv);
        println!("Attack DV: {}", attack_dv);
        println!("Defense DV: {}", defense_dv);
        println!("Speed DV: {}", speed_dv);
        println!("Special DV: {}", special_dv);

        println!("\n--- Trainer ---");
        println!("Original Trainer ID: {}", trainer_id);

        println!("\n--- Raw Pokemon Data ---");

        for (row, bytes) in pokemon.chunks(8).enumerate() {
            print!("{:02X}: ", row * 8);

            for byte in bytes {
                print!("{:02X} ", byte);
            }

            println!();
        }
    }

    Ok(())
}