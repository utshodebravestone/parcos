use crate::{error::Error, parser::Parser, stream::Streamable};

/// # Or
/// For choosing between two parser (P1 & P2).
pub struct Or<P1, P2>(P1, P2);

/// For constructing Or.
pub fn or<P1, P2>(p1: P1, p2: P2) -> Or<P1, P2> {
    Or(p1, p2)
}

impl<I, O, P1: Parser<I, O>, P2: Parser<I, O>> Parser<I, O> for Or<P1, P2> {
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<O, Error<I>>)
    where
        Self: Sized,
    {
        match self.0.parse_impl(stream, errors) {
            (p, Ok(o)) => (p, Ok(o)),
            (0, Err(e)) => match self.1.parse_impl(stream, errors) {
                (p, Ok(o)) => (p, Ok(o)),
                (p, Err(ne)) => (p, Err(e.merge(ne))),
            },
            e => e,
        }
    }
}
