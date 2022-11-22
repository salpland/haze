use colored::ColoredString;
use miette::Diagnostic;
use std::{io, path::PathBuf};
use thiserror::Error;

pub type HazeResult<T> = Result<T, HazeError>;

#[derive(Error, Diagnostic, Debug)]
pub enum HazeError {
    #[error("cannot read the config file at `{1}`")]
    #[diagnostic(help("{0}"))]
    CannotReadConfig(io::ErrorKind, ColoredString),

    #[error("the config file at `{1}` was not parsed")]
    #[diagnostic(help("{0}"))]
    CannotParseConfig(serde_json::Error, ColoredString),

    #[error("the `worlds` config property is empty")]
    #[diagnostic(help("the property must include at least one pattern"))]
    EmptyWorldsProperty,

    #[error("invalid glob pattern in `worlds` property `{1}`")]
    #[diagnostic(help("{0}"))]
    InvalidWorldsGlobPattern(glob::PatternError, String),

    #[error("cannot find a worlds directory from the defined patterns `{1:?}`")]
    #[diagnostic(help("{0}"))]
    WorldsDirectoryNotFound(io::ErrorKind, Vec<String>),

    #[error("cannot find the world `{1}` in local worlds directory")]
    #[diagnostic(help("available worlds: {0:?}"))]
    WorldNotFound(Vec<PathBuf>, String),

    #[error("unable to find the local appdata directory")]
    #[diagnostic(help("{0}"))]
    CannotFindLocalAppData(io::ErrorKind),

    #[error("couldn't copy the world `{1}`")]
    #[diagnostic(help("{0}"))]
    CannotCopyWorld(io::ErrorKind, ColoredString),

    #[error("cannot overwrite the world `{0}` as it already exists in `com.mojang`")]
    #[diagnostic(help("do \"haze test --overwrite {0}\" if you want to overwrite"))]
    CannotOverwriteWorld(ColoredString),
}
