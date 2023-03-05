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
    /// Copy a world from the project's worlds directory to "minecraftWorlds"
    Export {
        /// The name of the world to export
        name: String,
        /// Overwrite an already existing world with the same name
        #[arg(short, long)]
        overwrite: bool,
        /// The path to export the world to.
        ///
        /// Defaults to "stable".
        ///
        /// A custom path must point to a directory that contains your worlds.
        #[arg(short, long, default_value = "stable")]
        path: String,
    },
    /// Copy a world from "minecraftWorlds" to the project's worlds directory
    Import {
        /// The name of the world to import
        name: String,
        /// The path to import the world from.
        ///
        /// Defaults to "stable".
        ///
        /// A custom path must point to a directory that contains your worlds.
        #[arg(short, long, default_value = "stable")]
        path: String,
    },
    /// List all worlds in the project's worlds directory
    List,
}
