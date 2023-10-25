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

pub fn open_market() {
		// TODO: Create the Market (Structs/Enums?)
		println!("Welcome to the Market!");
}
