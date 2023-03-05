mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Command};

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let config = config::load(cli.config)?;
    match cli.command {
        Command::Export { name, overwrite } => haze_core::export(name, config.worlds, overwrite)?,
        Command::Import { name } => haze_core::import(name, config.worlds)?,
        Command::List => {
            let worlds = haze_core::all_worlds(config.worlds)?;

            println!("Available Worlds:");

            for world in worlds {
                println!("  > {world}");
            }
        }
    }
    Ok(())
}
