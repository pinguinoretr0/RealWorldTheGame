use crate::Output;
use crate::game::{
    market::NFT,
    market::crypto_to_usd,
    game::cal_rent,
    game::run_main,
    game::hit_return,
    data::PlayerData,
};
use std::io::{ self, Write };
use std::process::Command;
use rand::Rng;

fn get_usr(output: &dyn Output) -> PlayerData {
    let mut username = String::new();
    output.print("Enter your username:\n(MAX is 10 characters)");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    username = username.trim().to_string().to_lowercase();
    username = username.replace(" ", "_");
    
    if username.len() > 10 {
        output.print("\nYour username exceeds 10 characters.\nTruncated to the first 10 characters.\n");
        username.truncate(10);
    } else if username.is_empty() {
        output.print("\nUsername must not be empty!\n");
        return get_usr(output);
    }

    output.print(&format!("Username: {}", username));

    loop {
        output.print("\nIs this correct? [Y/n]");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        if response.is_empty() || response.eq_ignore_ascii_case("Y") {
            return PlayerData {
                username,
                bank: 0,
                ..Default::default()
            };
        } else if response.eq_ignore_ascii_case("n") {
            return get_usr(output);
        } else {
            output.print("Invalid input. Please enter 'Y' or 'n'.");
        }
    }
}

fn cal_intro_debt(player: &PlayerData, output: &dyn Output) -> PlayerData {
    let mut updated_player = player.clone();
    let mut rng = rand::thread_rng();
    let mut rolls_list = [0; 3];
    let (irs_debt, cartel_debt, rent);

    output.print("Rolling Die...");
    rolls_list[0] = rng.gen_range(1..=20);
    output.print(&format!("> {} rolled a: {}\n", player.username, rolls_list[0]));

    output.print("Rolling Die...");
    rolls_list[1] = rng.gen_range(1..=20);
    output.print(&format!("> {} rolled a: {}\n", player.username, rolls_list[1]));

    irs_debt = crypto_to_usd(rolls_list[0] * rolls_list[1], 0);
    updated_player.irsdebt = irs_debt;
    output.print(&format!("{} owes {} USD to the IRS...\n", player.username, irs_debt));

    output.print("Rolling Die...");
    rolls_list[2] = rng.gen_range(1..=20);
    output.print(&format!("> {} rolled a: {}", player.username, rolls_list[2]));

    cartel_debt = crypto_to_usd(rolls_list[2] * 3, 0);
    updated_player.carteldebt = cartel_debt; 
    output.print(&format!("{} owes {} USD to the Cartel...\n", player.username, cartel_debt));

    output.print(&format!("Calculating {}'s rent total...\n", player.username));
    rent = cal_rent(&player.username, output);
    updated_player.rent = rent;
    output.print(&format!("{}'s rent is {} USD.\n", player.username, rent));


    updated_player
}

pub fn run_intro(nft: &mut NFT, output: &dyn Output) {
    let mut clear_screen = Command::new("clear");
    clear_screen.status().expect("Process failed to execute");
    let player = get_usr(output);
    let player = cal_intro_debt(&player, output);
    hit_return(output);
    clear_screen.status().expect("Process failed to execute");
    run_main(&player, nft, output);
}
