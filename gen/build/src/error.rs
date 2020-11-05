use crate::gen::fs;
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::path::Path;

pub(super) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(super) enum Error {
    NoEnv(OsString),
    Fs(fs::Error),
    ExportedDirNotAbsolute(&'static Path),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoEnv(var) => {
                write!(f, "missing {} environment variable", var.to_string_lossy())
            }
            Error::Fs(err) => err.fmt(f),
            Error::ExportedDirNotAbsolute(path) => write!(
                f,
                "element of CFG.exported_header_dirs must be absolute path, but was: {:?}",
                path,
            ),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Fs(err) => err.source(),
            _ => None,
        }
    }
}

impl From<fs::Error> for Error {
    fn from(err: fs::Error) -> Self {
        Error::Fs(err)
    }
}
