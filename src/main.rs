use clap::Parser;
use crate::game::{
    market::NFT,
    intro::run_intro,
    game::run_main,
    game::open_manual,
    data::GameData,
    data::load_game
};
use crate::tui::{
    ui::launch_tui
};
use std::{
    io,
    io::Write,
};
use log::info;

mod game;
mod tui;

/// Real World: The Game CLI 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run the game (Options: 0 - TUI 1 - CLI) 
    #[arg(short, long)]
    play: Option<u8>,
    /// Load your save file (FILEPATH)
    #[arg(short, long)]
    load: Option<String>
    // /// Checks the bank account of a player (in progress)
    // #[arg(short, long)]
    // check_bank: String
}

pub trait Output {
    fn print(&self, message: &str);
}

pub struct CliOutput;
pub struct TuiOutput;

impl Output for CliOutput {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}

impl Output for TuiOutput {
    fn print(&self, message: &str) {
        info!("{}", message);
    }
}

fn display_help() {
    println!("Welcome to Real World: The Game!\nA simple game created by TheLinuxPirate.");
    println!("\nYou have chosen the Help function please type one of these options:");

    println!("(1: Opens the game manual (Default); 2: Gives a basic summary of the game; 3: Quit)\n");
    print!("> ");

    loop {
        let mut response = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim().to_string();

        if response.is_empty() || response.eq_ignore_ascii_case("1") {
            let output = CliOutput;
            open_manual(&output);
            break;
        } else if response.eq_ignore_ascii_case("2") {
            println!("\nReal World: The Game is a game where you are placed in \
                      a fictional world\nwhere you owe lots of debt, and you have \
                      to pay off within a time limit of 30 days.\nIn this world you \
                      an artist whom creates NFTs;\nthen sells them to get enough money to pay off your debt. \
                      \nThese debts range between the IRS, Rent, and Cartel Debt. \
                      \nYou play this game via a terminal \
                      where your scores are saved.\nMaybe you can try competing for a speedrun?");
            println!("\n[ EXITING ]");
            break;
        } else if response.eq_ignore_ascii_case("3") {
            println!("[ EXITING ]");
            break;
        } else {
            println!("Invalid input. Please enter a valid option");
        }
    }
}

fn main() {
    let args = Args::parse();
    let gamedata = GameData::default();
    let mut nft = NFT::default();
    
    let output: Box<dyn Output> = match args.play {
        Some(0) => {
            launch_tui().expect("Failure to launch UI");
            Box::new(TuiOutput)
        },
        Some(1) => {
            if args.load.is_none() {
                let cli = CliOutput;
                run_intro(&mut nft, &cli);
            }
            Box::new(CliOutput)
        },
        _ => {
            eprintln!("Invalid play mode. Expected 0 for TUI or 1 for CLI.");
            return;
        }
    };

    if let Some(filename) = args.load {
        match load_game(filename) {
            Ok(player) => {
                println!("Player data loaded successfully:");
                println!("Username: {}", player.username);
                println!("Bank: {}", player.bank);

                println!("Starting Game...");
                run_main(&player, gamedata, &mut nft, &*output);
            }
            Err(err) => eprintln!("Error loading player data: {}", err),
        }
    }
}
