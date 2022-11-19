use colored::ColoredString;
use miette::Diagnostic;
use std::io;
use thiserror::Error;

pub type HazeResult<T> = Result<T, HazeError>;

#[derive(Error, Diagnostic, Debug)]
pub enum HazeError {
    #[error("couldn't read config file at `{1}`")]
    #[diagnostic(code(haze::config::io_error), help("{0}"))]
    ConfigRead(io::Error, ColoredString),

    #[error("config file at `{1}` was not parsed")]
    #[diagnostic(code(haze::config::parse_error), help("{0}"))]
    ConfigParse(serde_json::Error, ColoredString),

    #[error("unable to find the local appdata directory")]
    #[diagnostic(code(haze::world::local_appdata_error), help("{0}"))]
    LocalAppData(io::Error),

    #[error("couldn't copy the world `{1}`")]
    #[diagnostic(code(haze::world::copy_error), help("{0}"))]
    WorldCopy(io::Error, ColoredString),

    #[error("cannot overwrite the world `{0}` as it already exists in `com.mojang`")]
    #[diagnostic(
        code(haze::world::cannot_overwrite_world),
        help("do \"haze test --overwrite {0}\" if you want to overwrite")
    )]
    OverwriteWorld(ColoredString),
}
