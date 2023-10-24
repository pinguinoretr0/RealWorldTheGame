use crate::game::market::{
		crypto_to_usd,
		open_market
};
use crate::game::data::PlayerData;
use rand::Rng;
use std::io;

pub fn cal_rent(username: &str) -> i32 {
    let mut rng = rand::thread_rng();
    let card1 = rng.gen_range(1..=10);
    let card2 = rng.gen_range(1..=10);
    let card3 = rng.gen_range(1..=10);

    println!("Generating Rent Cards...\n");
    println!("> {}'s first card has a value of: {}", username, card1);
    println!("> {}'s second card has a value of: {}", username, card2);
    println!("> {}'s third card has a value of: {}", username, card3);

    let rent = crypto_to_usd(card1 + card2 + card3, 1);

		rent.try_into().unwrap()
}

pub fn hit_return() -> String {
		let mut turn = String::new();
		println!("Hit <Return> to continue: ");
		io::stdin().read_line(&mut turn).unwrap();
		let turn = turn.trim();

		if turn.is_empty() {
				return "[ O K ]".to_string();
		} else {
				return "[ O K ]".to_string();
		}
}

pub fn run_main(player: &PlayerData) {
		println!("> Hello {};\nYour introduction has been completed. \
							Its now time for you to start the Main Game!\n(Type: \"help\" for the manual; \"list\" for a list of options)", player.username);
		take_turn(&player);
}

fn take_turn(player: &PlayerData) {
		let mut turn_counter: u8 = 0;

		loop {
				let mut turn = String::new();
				print!("> ");
				io::stdin().read_line(&mut turn).unwrap();
				let turn = turn.trim();

				if turn.is_empty() {
						println!("Please input a valid option!");
						return take_turn(&player);
				} else if turn == "list" || turn == "List" {
						let options = ["Option 1: Sample"];
						println!("{}\n", options[0]);
						return take_turn(&player);
				} else if turn == "market" || turn == "Market" {
						open_market();
						return take_turn(&player);
				}



				else {
						println!("Please input a valid option!");
						return take_turn(&player);
				}
		}
}
