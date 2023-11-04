use crate::game::data::PlayerData;
use std::io;
use rand::Rng;
use prettytable::{
		Table,
		row,
};

#[derive(Default)]
pub struct NFT {
		limit: u32,
		value: usize,
		trash: bool
}

struct Stock {
    title: String,
    price: u8,
    desc: String,
    buff: String,
    stock: StockConditions
}

enum StockConditions {
    InStock,
    OutOfStock,
    Limit(u16),
}

pub fn crypto_to_usd(x: u32, c: u8) -> u128 {
    let exchange_rate = match c {
        0 => 6500, // BTC
        1 => 4891, // ETH
        2 => 517,  // XMR
        3 => 413,  // LTC
        _ => {
            eprintln!("Conversion Error...");
            std::process::exit(1);
        }
    };

    (x * exchange_rate).into()
}

pub fn create_nft(nft: &NFT) -> bool {
		let mut rng = rand::thread_rng();
		let mut rolls_list = [0; 2];
		let mut hours: u8;

		println!("Creating NFT...");
		rolls_list[0] = rng.gen_range(1..=6);
		rolls_list[1] = rng.gen_range(1..=6);

		println!("Set the price of your NFT");
		let mut nft_price = String::new();
		io::stdin().read_line(&mut nft_price).unwrap();
		println!("You typed: {nft_price}");

		// R1 | Determines if purchase was successful
    let purchase_success = match rolls_list[0] {
        1..=3 => true,
        4..=6 => {
            println!("Purchase Failed!");
            return false;
        }
        _ => unreachable!(),
    };

    // R2 | Determines price adjustment 
    let price_adjustment = match rolls_list[1] {
        1..=3 => {
            println!("Regular Price");
            1.0
        }
        4..=6 => {
            println!("Double Price");
            2.0
        }
        _ => unreachable!(),
    };

    // If the purchase is successful, proceed with creating and pricing the NFT
    // if purchase_success {
    //     // Logic for creating NFT (you might want to update the NFT struct fields accordingly)
    //     // For example:
    //     // nft.limit = some_value;
    //     // nft.value = some_value;
    //     // nft.trash = some_value;

    //     // Logic for determining the NFT price based on the price adjustment
    //     let base_price = 15; // You can replace this with the actual base price
    //     let nft_price = (base_price as f64 * price_adjustment) as u32;

    //     // Logic for determining the time spent on creating the NFT
    //     // For example:
    //     // hours = some_value;

    //     // Display the results
    //     println!("NFT Created Successfully!");
    //     println!("NFT Price: {} Litecoin", nft_price);
    //     println!("Time Spent: {} hours", hours);
    // } else {
    //     println!("No NFT created due to failed purchase!");
    // }

		purchase_success
}

pub fn open_market(player: &PlayerData) {
    let mut market: Vec<Stock> = Vec::new();
    let mut menu = Table::new();

    // Market items
    market.push(Stock {
        title: String::from("Red Bull"),
        price: 4,
        desc: String::from("A very tasty energy drink"),
        buff: String::from("5 NFTs Buff"),
        stock: StockConditions::InStock,
    });

    market.push(Stock {
        title: String::from("Monster"),
        price: 3,
        desc: String::from("Nasty ass energy drink"),
        buff: String::from("3 NFTs Buff"),
        stock: StockConditions::InStock,
    });
    
    // Display
    println!("\nWelcome to the Market {}!", player.username);
    println!("Here are your options:\n");

    // Header
    menu.add_row(row!["Item Name", "Price (USD)", "Description", "Effect", "Stock"]);
    // Options
    for item in &market {
        let stock_display = match &item.stock {
            StockConditions::InStock => "True".to_string(),
            StockConditions::OutOfStock => "0".to_string(),
            StockConditions::Limit(count) => count.to_string(),
        };

        menu.add_row(row![&item.title, &item.price, &item.desc, &item.buff, &stock_display]);
    }
    menu.printstd();
}
