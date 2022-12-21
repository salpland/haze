use clap::Parser;
use cli::{Cli, Command};
use haze_core::{config, world};

mod cli;

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let config = config::load(cli.config)?;
    match cli.command {
        Command::Export { name, overwrite } => world::export(name, config.worlds, overwrite)?,
        Command::Import { name } => world::import(name, config.worlds)?,
        Command::List => {
            let worlds = world::all_worlds(config.worlds)?;

            println!("Available Worlds:");

            for world in worlds {
                println!("  > {world}");
            }
        }
    }
    Ok(())
}
