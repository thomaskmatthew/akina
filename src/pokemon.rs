
pub fn species_name(id: u8) -> &'static str {
    match id {
        153 => "Bulbasaur",
        9 => "Ivysaur",
        154 => "Venusaur",
        176 => "Charmander",
        178 => "Charmeleon",
        180 => "Charizard",
        177 => "Squirtle",
        179 => "Wartortle",
        28 => "Blastoise",
        84 => "Pikachu",
        85 => "Raichu",
        148 => "Abra",
        149 => "Alakazam",
        147 => "Haunter",
        185 => "Oddish",
        _ => "Unknown Pokemon",
    }
}