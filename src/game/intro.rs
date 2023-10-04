use crate::game::{ market::crypto_to_usd, game::cal_rent };
use std::io::{self, Write};
use rand::Rng;

fn get_usr() -> String {
    let mut username = String::new();
    print!("Enter your username:\n(MAX is 10 characters)\n> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    username = username.trim().to_string();

    if username.len() > 10 {
        println!("\nYour username exceeds 10 characters.\nTruncated to the first 10 characters.\n");
        username.truncate(10);
    }

    println!("Username: {}", username);

    loop {
        print!("\nIs this correct? [Y/n]\t> ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        if response.is_empty() || response.eq_ignore_ascii_case("Y") {
            return username;
        } else if response.eq_ignore_ascii_case("n") {
            println!("Enter your new username: ");
            return get_usr();
        } else {
            println!("Invalid input. Please enter 'Y' or 'n'.");
        }
    }
}

fn cal_intro_debt(username: &str) {
    let mut rng = rand::thread_rng();
    let mut rolls_list = [0; 3];
    let mut irs_debt;
    let mut cartel_debt;

    println!("Rolling Die...");
    rolls_list[0] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}\n", username, rolls_list[0]);

    println!("Rolling Die...");
    rolls_list[1] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}\n", username, rolls_list[1]);

    irs_debt = crypto_to_usd(rolls_list[0] * rolls_list[1], 0);
    println!("{} owes {} USD to the IRS...\n", username, irs_debt);

    println!("Rolling Die...");
    rolls_list[2] = rng.gen_range(1..=20);
    println!("> {} rolled a: {}", username, rolls_list[2]);
    cartel_debt = crypto_to_usd(rolls_list[2] * 3, 0);
    println!("{} owes {} USD to the Cartel...\n", username, cartel_debt);

    println!("Calculating {}'s rent total...\n", username);
    let rent = cal_rent(username);
    println!("{}'s rent is {} USD.", username, rent);
}

pub fn run_intro() {
    let username = get_usr();
    cal_intro_debt(&username);
}
