//! ParCos
//!
//! Parser Combinator Library.

use std::fmt;

/// This is where all the combinator lives.
pub mod combinators;

/// Combinators generates this object.
#[derive(Debug, PartialEq)]
pub enum ParsedObject {
    Char(char),
    String(String),
}

/// Combinators returns this if it fails.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// What was expected, got what, in where.
    Unexpected(String, String, usize),
    /// Not enough input to consume from. Tried to consume what and where.
    NotEnoughInput(String, usize),
}

impl<'a> fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Unexpected(expected, got, location) => {
                write!(f, "expected < {expected} >, got < {got} > in {location}")
            }
            ParseError::NotEnoughInput(expected, location) => write!(
                f,
                "expected {expected} but wasn't enough input to consume from in {location}"
            ),
        }
    }
}

/// Result produced by parser: unconsumed input, what was parsed or error why it failed.
pub type ParseResult = Result<(String, ParsedObject), ParseError>;

/// This what a parser is.
pub type Parser = Box<dyn Fn(&str) -> ParseResult>;
