use std::net::AddrParseError;

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
