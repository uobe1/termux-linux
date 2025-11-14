mod cli;
mod distro;
mod utils;
mod installer;
mod system;
mod config;
mod ui;

use cli::run_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli()
}