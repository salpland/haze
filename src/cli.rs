use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set a path to the config file
    #[arg(short, long, value_name = "PATH", default_value = "config.json")]
    pub config: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Copy a world from the project's worlds folder into "minecraftWorlds"
    Test { name: String },
    /// Copy a world from "minecraftWorlds" into the project's worlds folder
    Save { name: String },
}
