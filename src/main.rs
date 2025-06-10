#[cfg(feature = "cli")]
use bfx::cli::main as cli_main;

#[cfg(feature = "cli")]
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(cli_main());
}

#[cfg(not(feature = "cli"))]
fn main() {
    {
        eprintln!("This binary is only available with the 'cli' feature enabled.");
        std::process::exit(1);
    }
}
