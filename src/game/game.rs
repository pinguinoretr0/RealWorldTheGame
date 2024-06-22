use crate::{CliOutput, Output};
use crate::game::{
    market::{NFT, create_nft, crypto_to_usd, open_market},
    data::{PlayerData, GameData, save_game}
};
use std::{
    process::Command,
    io
};
use prettytable::{Table, row};
use rand::Rng;
use std::fs;
use termimad::MadSkin;

pub fn cal_rent(username: &str, output: &dyn Output) -> u32 {
    let mut rng = rand::thread_rng();
    let card1 = rng.gen_range(1..=10);
    let card2 = rng.gen_range(1..=10);
    let card3 = rng.gen_range(1..=10);

    output.print("Generating Rent Cards...\n");
    output.print(&format!("> {}'s first card has a value of: {}", username, card1));
    output.print(&format!("> {}'s second card has a value of: {}", username, card2));
    output.print(&format!("> {}'s third card has a value of: {}", username, card3));

    let rent = crypto_to_usd(card1 + card2 + card3, 1);

    rent.try_into().unwrap()
}

pub fn hit_return(output: &dyn Output) -> String {
    let mut turn = String::new();
    output.print("Hit <Return> to continue: ");
    io::stdin().read_line(&mut turn).unwrap();
    let turn = turn.trim();

    if turn.is_empty() {
        "[ O K ]".to_string()
    } else {
        "[ O K ]".to_string()
    }
}

pub fn open_manual(output: &dyn Output) {
    let skin = MadSkin::default();

    let markdown_content = fs::read_to_string("example.md")
        .expect("Failed to read the Markdown file");

    skin.print_text(&markdown_content);
}

pub fn run_main(player: &PlayerData, game: GameData, nft: &mut NFT, output: &dyn Output) {
    let day_counter: u8 = 0;
    let new_game = game.clone();

    output.print(&format!("> Hello {};\nYour introduction has been completed. \
              Its now time for you to start the Main Game!\n(Type: \"help\" for the manual; \"list\" for a list of options)", player.username));

    if day_counter <= 30 {
        let turn_value: u8 = cal_limit();

        for _ in 0..turn_value {
            take_turn(player, new_game.clone(), nft, output);
        }   
    } else {
        output.print("Game Over!");
    }
}

fn cal_limit() -> u8 {
    // TODO: Make a better counter that implements "day_counter"
    30
}

fn take_turn(
    player: &PlayerData,
    game: GameData,
    nft: &mut NFT,
    output: &dyn Output
) -> u8 {
    let mut turn_counter: u8 = 0;

    loop {
        output.print("");
        let mut turn = String::new();
        io::stdin().read_line(&mut turn).unwrap();
        let turn = turn.trim();

        match turn.to_lowercase().as_str() {
            "" => {
                output.print("Please input a valid option!");
                return take_turn(player, game, nft, output);
            }
            "list" | "help" => {
                let mut list = Table::new();
                output.print("");

                // Header
                list.add_row(row!["Command", "Description"]);
                // Options
                list.add_row(row!["\"list\"", "Prints this command"]);
                list.add_row(row!["\"market\"", "Opens the Market Menu"]);
                list.add_row(row!["\"create\"", "Creates NFTs (uses 1 turn) N/A"]);
                list.add_row(row!["\"clear\"", "Clears the screen"]);
                list.add_row(row!["\"check\"", "Displays turns left & time limit N/A"]);
                list.add_row(row!["\"end\"", "Manually ends current turn N/A"]);
                list.add_row(row!["\"save\"", "Saves the current game"]);
                list.add_row(row!["\"quit\"", "Quits game (without saving!) N/A"]);
  
                // Table setup...
                list.printstd();
                return take_turn(player, game, nft, output);
            }
            "market" => {
                open_market(player, output);
                return take_turn(player, game, nft, output);
            }
            "save" => {
                output.print("\nSaving Game Data...");
                save_game(player, &game);
                return take_turn(player, game, nft, output);
            }
            "clear" => {
                let mut clear_screen = Command::new("clear");
                clear_screen.status().expect("Process failed to execute");
                return take_turn(player, game, nft, output);
            }
            "create" => {
                // TODO: Set up NFT building + stats
                create_nft(nft, game.clone(), output);
                return take_turn(player, game, nft, output);
            }
            "end" => {
                output.print("You have manually ended your turn...");
                turn_counter += 1;
                output.print(&format!("Turn Counter: {}", turn_counter));
                break;
            }
            "quit" => {
                output.print("[ O K ]");
                output.print("Exiting Game...");
                break;
            }
            _ => {
                output.print("Please input a valid option!");
                return take_turn(player, game, nft, output);
            }
        }
    }

    turn_counter
}

pub fn end_game(output: &dyn Output) {
    output.print("Your game has ended!");
}
