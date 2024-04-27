use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug , PartialEq, Eq)]
pub enum Error {
    #[error("Missing Prefix of Slice")]
    NoPrefixOfSlice,
    #[error("Missing Suffix of Slice")]
    NoAppendixOfSlice,
    #[error("Parse Int Failed with {0}")]
    ParseIntError(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
