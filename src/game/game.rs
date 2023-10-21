use crate::game::market::crypto_to_usd;
use crate::game::data::PlayerData;
use rand::Rng;

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
    rent
}

pub fn run_main(player: &PlayerData) {
		println!("> Hello {}; Your introduction has been completed. \
							Its now time for you to start the main game;\n(Type: \"help\" for the manual)", player.username);
}
