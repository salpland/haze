mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Command};
use miette::Result;
use owo_colors::OwoColorize;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load(cli.config)?;
    match cli.command {
        Command::Export {
            name,
            overwrite,
            path,
        } => {
            haze_core::export(&name, &config.worlds, &path, overwrite)?;
            if overwrite {
                println!(
                    "{} world \"{}\" in the \"minecraftWorlds\" directory ({})",
                    "Updated".bold().green(),
                    name,
                    "overwrite".red()
                );
            } else {
                println!(
                    "{} world \"{}\" to the \"minecraftWorlds\" directory for testing",
                    "Copied".bold().green(),
                    name
                );
            }
        }
        Command::Import { name, path } => {
            haze_core::import(&name, &config.worlds, &path)?;
            println!(
                "{} world \"{}\" to the local worlds directory",
                "Saved".bold().green(),
                name
            );
        }
        Command::List => {
            println!("Available worlds:");
            for world in haze_core::all_worlds(&config.worlds)? {
                println!("  {} {world}", ">".cyan());
            }
        }
    }

    Ok(())
}
