use clap::Parser;
use cli::{Cli, Command};
use miette::Result;

mod cli;
mod config;
mod error;
mod world;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::load(&cli.config)?;
    match cli.command {
        Command::Test { name, overwrite } => {
            world::test(&name, &config.packs.world_template, overwrite)?
        }
        Command::Save { name } => world::save(&name, &config.packs.world_template)?,
    }

    Ok(())
}
