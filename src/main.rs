use clap::Parser;
use cli::{Cli, Command};
use miette::Result;

mod cli;
mod config;
mod error;
mod world;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Test { name } => {
            println!("{}", name);
        }
        Command::Save { name } => {
            println!("{}", name);
        }
    }

    let config = config::load(&cli.config)?;
    println!("{}", config.packs.world_template);

    Ok(())
}
