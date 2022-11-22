use clap::Parser;
use cli::{Cli, Command};

mod cli;
mod config;
mod error;
mod world;

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let config = config::load(cli.config)?;
    match cli.command {
        Command::Test { name, overwrite } => world::test(name, config.worlds, overwrite)?,
        Command::Save { name } => world::save(name, config.worlds)?,
    }
    Ok(())
}
