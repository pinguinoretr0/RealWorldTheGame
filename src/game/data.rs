use serde_json::json;

pub struct PlayerData {
		pub username: String,
		pub bank: usize
}

fn save_playerdata(player: &PlayerData) {
		let player_data = json!({
				"username": player.username,
				"bank_account": player.bank
		});
}
