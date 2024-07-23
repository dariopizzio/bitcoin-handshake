use std::string::FromUtf8Error;
use std::{array::TryFromSliceError, net::AddrParseError};

use anyhow::anyhow;
use anyhow::Error;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum HandshakeError {
    #[error("Parse error: {0}")]
    ParseError(AddrParseError),
    #[error("There was an error decoding bytes: {0}")]
    ByteDecodingError(Error),
    #[error("There was an error calculating the checksum")]
    ChecksumError,
    #[error("Invalid header command")]
    InvalidHeaderCommand,
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
