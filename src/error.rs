use std::{io, path::PathBuf};
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum RenameBooksError {
    #[error("Home directory must exist")]
    HomeError,

    #[error("Temporary error: {path}")]
    TemporaryError { path: PathBuf },

    #[error("Temporary is not a directory: {path}")]
    TemporaryNotADirectoryError { path: PathBuf },

    #[error("IO error while reading directory: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
}

impl RenameBooksError {
    pub fn home_directory_error() -> Self {
        RenameBooksError::HomeError
    }

    pub fn invalid_directory(path: PathBuf) -> Self {
        RenameBooksError::TemporaryError { path }
    }
}
