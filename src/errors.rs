use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeadTailError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("File watcher error: {0}")]
    FileWatcherError(#[from] notify::Error),
}

pub type Result<T> = std::result::Result<T, HeadTailError>;
