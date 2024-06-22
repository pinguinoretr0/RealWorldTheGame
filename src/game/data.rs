use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    error::Error,
};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub username: String,
    pub bank: usize,

    pub irsdebt: u32,
    pub carteldebt: u32,
    pub rent: u32,

    pub current_hrs: u8,
    pub current_day: u8,
}

fn save_playerdata(
    player: &PlayerData,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let toml_data = toml::to_string(&player)?;
    let mut file = File::create(filename)?;
    file.write_all(toml_data.as_bytes())?;

    Ok(())
}

pub fn save_game(player: &PlayerData) {
    let player = PlayerData {
        username: player.username.to_string(),
        bank: player.bank,

        irsdebt: player.irsdebt,
        carteldebt: player.carteldebt,
        rent: player.rent,

        current_day: player.current_day,
        current_hrs: player.current_hrs,
    };

    // once ready store this file in ~/.local/share/rwg/
    // for Windows C:\Users\[your username]\AppData

    if let Err(err) = save_playerdata(&player, "player_data.toml") {
        eprintln!("Error saving player data: {}", err);
    } else {
        println!("Player data saved successfully!");
    }
}

pub fn load_game(filename: String) -> Result<PlayerData, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let playerdata: PlayerData = toml::from_str(&buffer)?;
    Ok(playerdata)
}
