pub fn crypto_to_usd(x: i32, c: i32) -> i32 {
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

    x * er
}
