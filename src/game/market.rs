use crate::game::data::PlayerData;
use prettytable::{
    Table,
    row,
};

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
    let er = match c {
        0 => 6500, // BTC
        1 => 4891, // ETH
        2 => 517,  // XMR
        3 => 413,  // LTC
        _ => {
            eprintln!("Conversion Error...");
            std::process::exit(1);
        }
    };

    (x * er).into()
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
