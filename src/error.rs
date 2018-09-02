use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    ParsingFailed(String),
    VersionNotFound(String),
    VersionNotIncreasing(String),
    IoError(String),
}

pub type GradleResult<T> = Result<T, Error>;

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Error {
        let reason = format!("failed to read line: {:?}", io_error);
        Error::IoError(reason)
    }
}
