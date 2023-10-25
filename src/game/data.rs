use serde_json::json;
use std::fs::File;

pub struct PlayerData {
		pub username: String,
		pub bank: usize
}

fn save_playerdata(player: &PlayerData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
		let player_data = json!({
				"username": player.username,
				"bank_account": player.bank
		});

		let file = File::create(filename)?;

    serde_json::to_writer(file, &player_data)?;

    Ok(())
}

pub fn data_backtrace(player: &PlayerData) {
		let player = PlayerData {
        username: player.username.to_string(),
        bank: 0,
    };

    if let Err(err) = save_playerdata(&player, "player_data.json") {
        eprintln!("Error saving player data: {}", err);
    } else {
        println!("Player data saved successfully!");
    }
}
