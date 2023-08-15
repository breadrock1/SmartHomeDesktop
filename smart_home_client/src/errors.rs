use std::io;
use thiserror::Error;

pub type HandleResult = Result<(), ConnectError>;
pub type ConnectResult<T> = Result<T, ConnectError>;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Unexpected handshake response: {0}")]
    BadHandshake(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Runtime exception")]
    Other(String),
}

pub type RecvResult = Result<String, RecvError>;

#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Bad encoding exception")]
    BadEncoding,
    #[error("Failed while reading buffer")]
    ReadData(String),
}

pub type SendResult = Result<String, SendError>;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Bad encoding exception")]
    BadEncoding,
    #[error("Failed while writing buffer")]
    WriteData(String),
}
