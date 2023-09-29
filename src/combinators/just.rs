use crate::{error::Error, parser::Parser, stream::Streamable};

/// # Just
/// For parsing specific things (like operator, keyword etcetera).
#[derive(Debug)]
pub struct Just<Input>(Input);

impl<Input: Clone + PartialEq> Parser<Input, Input> for Just<Input> {
    fn parse_impl<Stream: Streamable<Input>>(
        &self,
        stream: &mut Stream,
        _: &mut Vec<Error<Input>>,
    ) -> (usize, Result<Input, Error<Input>>)
    where
        Self: Sized,
    {
        match stream.peek() {
            Some(x) if x == &self.0 => (1, Ok(stream.next().unwrap())),
            x => {
                let x = x.cloned();
                (
                    0,
                    Err(Error::Unexpected(stream.position(), self.0.clone(), x)),
                )
            }
        }
    }
}

/// For constructing Just
/// ```
/// use parcos::{parser::Parser, combinators::just};
///
/// let slash_parser = just('/');
/// let parsed = slash_parser.parse("/foo".chars());
///
/// assert!(parsed.is_ok());
/// assert_eq!(parsed.unwrap(), '/');
/// ```
pub fn just<I: Clone + PartialEq>(x: I) -> Just<I> {
    Just(x)
}
