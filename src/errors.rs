use std::io::Error;
use std::string::FromUtf8Error;
use std::{array::TryFromSliceError, net::AddrParseError};

use anyhow::anyhow;
use anyhow::Error as AnyhowError;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum HandshakeError {
    #[error("Parse error: {0}")]
    ParseError(AddrParseError),
    #[error("There was an error decoding bytes: {0}")]
    ByteDecodingError(AnyhowError),
    #[error("There was an error calculating the checksum")]
    ChecksumError,
    #[error("Invalid header command")]
    InvalidHeaderCommand,
    #[error("There was an error while reading from the socket")]
    BufferReadError(Error),
}

impl From<TryFromSliceError> for HandshakeError {
    fn from(e: TryFromSliceError) -> Self {
        HandshakeError::ByteDecodingError(anyhow!(e))
    }
}

impl From<FromUtf8Error> for HandshakeError {
    fn from(e: FromUtf8Error) -> Self {
        HandshakeError::ByteDecodingError(anyhow!(e))
    }
}

impl From<Error> for HandshakeError {
    fn from(e: Error) -> Self {
        HandshakeError::BufferReadError(e)
    }
}
