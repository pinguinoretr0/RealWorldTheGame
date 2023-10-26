use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::{
    fs::File,
    io::Read
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub username: String,
    pub bank: usize
}

pub struct GameData {
    
}

fn save_playerdata(player: &PlayerData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let player_data = json!({
        "username": player.username,
        "bank": player.bank
    });

    let file = File::create(filename)?;

    serde_json::to_writer(file, &player_data)?;

    Ok(())
}

pub fn save_game(player: &PlayerData) {
    let player = PlayerData {
        username: player.username.to_string(),
        bank: player.bank,
    };

    if let Err(err) = save_playerdata(&player, "player_data.json") {
        eprintln!("Error saving player data: {}", err);
    } else {
        println!("Player data saved successfully!");
    }
}

pub fn load_game(filename: String) -> Result<PlayerData, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let player_data: PlayerData = serde_json::from_str(&buffer)?;

    Ok(player_data)
}
