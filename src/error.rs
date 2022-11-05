use miette::Diagnostic;
use std::io;
use thiserror::Error;

pub type HazeResult<T> = Result<T, HazeError>;

#[derive(Error, Diagnostic, Debug)]
pub enum HazeError {
    #[error("couldn't read config file")]
    #[diagnostic(
        code(haze::config::io_error),
        help("check if the path for the config file is correct, or if the file actually exists")
    )]
    ConfigIo(#[from] io::Error),

    #[error("couldn't parse config file")]
    #[diagnostic(code(haze::config::parse_error))]
    ConfigParse(#[from] serde_json::Error),
}
