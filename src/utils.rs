use std::env;
use std::io::Write;
use std::path::PathBuf;

pub fn parse_ccy_from_symbol(symbol: &str) -> &str {
    match symbol.get(0..1) {
        Some("f") => &symbol[1..],
        Some("t") => {
            if let Some(idx) = symbol.find(":") {
                // tETH:USDT
                &symbol[idx + 1..]
            } else {
                // BTCUSD
                &symbol[4..]
            }
        }
        _ => symbol,
    }
}

pub fn home_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        env::var("USERPROFILE").map(PathBuf::from).ok()
    } else {
        env::var("HOME").map(PathBuf::from).ok()
    }
}

pub fn resolve_env_path_or_create() -> PathBuf {
    let path = PathBuf::from(".bfx_cli.env");
    if path.exists() {
        return path;
    }

    let user_home = home_dir().expect("Failed to get home directory");
    if !user_home.exists() {
        eprintln!("Home directory does not exist: {}", user_home.display());
        std::process::exit(1);
    }
    if !user_home.is_dir() {
        eprintln!("Home directory is not a directory: {}", user_home.display());
        std::process::exit(1);
    }

    let env_path = user_home.join(".bfx_cli.env");
    if env_path.exists() {
        return env_path;
    }

    // Create the .bfx_cli.env file if it doesn't exist, and ask for inputing api key and api secret
    let mut fs = std::fs::File::create(&env_path).expect("Failed to create .bfx_cli.env file");

    let mut api_key = String::new();
    let mut api_secret = String::new();

    println!("Please enter your Bitfinex API key:");
    std::io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read from stdin");

    println!("Please enter your Bitfinex API secret:");
    std::io::stdin()
        .read_line(&mut api_secret)
        .expect("Failed to read from stdin");

    fs.write(format!("API_KEY={api_key}API_SECRET={api_secret}").as_bytes())
        .expect("Failed to write content to env file");

    env_path
}
