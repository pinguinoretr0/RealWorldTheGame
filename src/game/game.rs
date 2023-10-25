use crate::game::{
    market::crypto_to_usd,
    market::open_market,
    data::PlayerData,
    data::save_game
};
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
    let day_counter: u8 = 0;

    println!("> Hello {};\nYour introduction has been completed. \
              Its now time for you to start the Main Game!\n(Type: \"help\" for the manual; \"list\" for a list of options)", player.username);

    if day_counter <= 30 {
        let turn_value: u8 = cal_limit();

        for _ in 0..turn_value {
            take_turn(&player);
        }   
    } else {
        println!("Game Over!");
    }
}

fn cal_limit() -> u8 {
    // TODO: Make a better counter that implements "day_counter"
    30
}

fn take_turn(player: &PlayerData) -> u8 {
    let mut turn_counter: u8 = 0;

    loop {
        print!("\n");
        let mut turn = String::new();
        io::stdin().read_line(&mut turn).unwrap();
        let turn = turn.trim();

        if turn.is_empty() {
            println!("Please input a valid option!");
            return take_turn(&player);
        } else if turn == "list" || turn == "List" {
            let options = [
                "\"list\": Lists options",
                "\"market\": Opens the Market Menu",
                "\"save\": Saves current game",
                "\"check\": Displays turns left & time limit",
                "\"create\": Creates NFTs (uses 1 turn)",
                "\"end\": Manually ends your turn"
            ];

            for option in options.iter() {
                println!("\n{}", option);
            }

						return take_turn(&player);
        } else if turn == "market" || turn == "Market" {
            open_market(&player);
            return take_turn(&player);
        } else if turn == "save" || turn == "Save" {
						println!("Saving Game Data...");
						save_game(&player);
						return take_turn(&player);
				}

				else if turn == "create" || turn == "Create" {
            // TODO: Set up NFT building + stats
            println!("*builds nft*");
            return take_turn(&player);
        } else if turn == "end" || turn == "End" {
            println!("You have manually ended your turn...");
            turn_counter += 1;
            println!("Turn Counter: {}", turn_counter);
            break;
        }
        else {
            println!("Please input a valid option!");
            return take_turn(&player);
        }
    }

    turn_counter
}
