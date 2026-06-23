use std::{error::Error, fmt::Display, io};

use crate::types::Type;

#[derive(Debug)]
pub enum DnsError {
    OutOfBytes { needed: usize, available: usize },
    PointerOutOfBounds{ offset: usize, buffer_len: usize },
    PointerLoop,
    IdMismatch { expected: u16, got: u16 },
    InvalidRdataLength { record_type: Type, length: u16 },
    Io(std::io::Error),
    UnknownType(u16),
    UnknownTypeString(String),
    UnknownClass(u16),
}

impl Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsError::OutOfBytes { needed, available } =>
                write!(f, "Out of Bytes needed: {needed}, available: {available}"),
            DnsError::PointerOutOfBounds { offset, buffer_len } => 
                write!(f, "Pointer offset out of Bounds, offset: {offset}, buffer length: {buffer_len}"),
            DnsError::PointerLoop => write!(f, "Error pointer keeps looping to the same place"),
            DnsError::IdMismatch { expected, got } => write!(f, "id mismatch: expected {expected}, got {got}"),
            DnsError::InvalidRdataLength { record_type, length } => 
                write!(f, "invalid RDATA length {length} for record type {record_type}"),
            DnsError::Io(error) => write!(f, "Error Input output {error}"),
            DnsError::UnknownType(value) => write!(f, "Error Unknown type {value}"),
            DnsError::UnknownTypeString(value) => write!(f, "Error Unknown type {value}"),
            DnsError::UnknownClass(value) => write!(f, "Error unknown class {value}")
        }
    }
}

impl Error for DnsError {

}

impl From<io::Error> for DnsError {
    fn from(value: io::Error) -> Self {
        DnsError::Io(value)
    }
}
