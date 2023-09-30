use crate::{error::Error, parser::Parser, stream::Streamable};

/// # Pred
/// For parsing with an predicate (P).
pub struct Pred<F>(F);

impl<I: Clone, F: Fn(&I) -> bool> Parser<I, I> for Pred<F> {
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        _: &mut Vec<Error<I>>,
    ) -> (usize, Result<I, Error<I>>)
    where
        Self: Sized,
    {
        match stream.peek() {
            Some(i) if (self.0)(i) => (1, Ok(stream.next().unwrap())),
            i => {
                let i = i.cloned();
                (0, Err(Error::Unexpected(stream.position(), vec![], i)))
            }
        }
    }
}

/// For constructing Pred.
/// ```
/// use parcos::{parser::Parser, combinators::pred};
///
/// let digit_parser = pred(|x: &char| x.is_ascii_digit());
/// let parsed = digit_parser.parse("10x".chars());
///
/// assert!(parsed.is_ok());
/// assert_eq!(parsed.unwrap(), '1');
/// ```
pub fn pred<I, F: Fn(&I) -> bool>(f: F) -> Pred<F> {
    Pred(f)
}
