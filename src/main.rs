use clap::Parser;
use crate::game::{
    market::NFT,
    intro::run_intro,
    game::run_main,
    data::GameData,
    data::load_game
};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{
    io,
    io::{Write, stdout, Result}
};

mod game;

/// CLI Arguments written for the game 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Runs the game 
    #[arg(short, long)]
    play: Option<u8>,
    /// Load save file
    #[arg(short, long)]
    load: Option<String>
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
            open_manual();
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

fn open_manual() {
    // Function will not run the game but open the manual
    println!("Manual CLI & Manual TUI");
}

fn launch_tui() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hellow Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let game = GameData::default();
    let mut nft = NFT::default();
    
    if let Some(filename) = args.load {
        match load_game(filename) {
            Ok(player) => {
                println!("Player data loaded successfully:");
                println!("Username: {}", player.username);
                println!("Bank: {}", player.bank);

                println!("Starting Game...");
                run_main(&player, game, &mut nft);
            }
            Err(err) => eprintln!("Error loading player data: {}", err),
        }
    }

    match args.play {
        Some(0) => launch_tui().expect("Failure to launch UI"),
        Some(1) => run_intro(&mut nft),
        _ => display_help(),
    }
}
