//! # Parcos
//!
//! Simple, Minimal Parser Combinator Library.

/// Contains Parcos Combinators Definitions and Implementations.
pub mod combinators;
/// Contains Parcos Error Definitions and Implementations.
pub mod error;
/// Contains Parcos Parser Definitions and Implementations.
pub mod parser;
/// Contains Parcos Stream Definitions and Implementations.
pub mod stream;

#[cfg(test)]
mod tests;
