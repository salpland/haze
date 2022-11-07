use miette::Diagnostic;
use std::io;
use thiserror::Error;

pub type HazeResult<T> = Result<T, HazeError>;

#[derive(Error, Diagnostic, Debug)]
pub enum HazeError {
    #[error("couldn't read config file")]
    #[diagnostic(code(haze::config::io_error), help("{0}"))]
    ConfigRead(io::Error),

    #[error("config file was not parsed")]
    #[diagnostic(code(haze::config::parse_error), help("{0}"))]
    ConfigParse(serde_json::Error),

    #[error("couldn't copy the world '{1}'")]
    #[diagnostic(code(haze::world::copy_error), help("{0}"))]
    WorldCopy(io::Error, String),

    #[error("unable to find local appdata directory")]
    #[diagnostic(code(haze::world::local_appdata_error), help("{0}"))]
    LocalAppData(io::Error),
}
