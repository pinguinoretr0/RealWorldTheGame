use crate::game::intro::run_intro;
use clap::Parser;
use std::io;
use std::io::Write;
mod game;

/// CLI Arguments written for the game 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
		/// Runs the game 
		#[arg(short, long, default_value_t = 0)]
		play: u8,
		// /// Checks the bank account of a player (in progress)
		// #[arg(short, long)]
		// check_bank: String
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
						open_manual_in_current();
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

fn open_manual_in_current() {
		// Function will not run the game but open the manual in the current terminal
		println!("=+TODO+=");
}

fn main() {
    let args = Args::parse();

		match args.play {
				0 => run_intro(),
				1 => display_help(),
				_ => display_help(),
		}
}
