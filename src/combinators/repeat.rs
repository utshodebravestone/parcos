use crate::{error::Error, parser::Parser, stream::Streamable};

/// # Repeat
/// For repeating a parser (P).
pub struct Repeat<P>(P);

impl<I, O, P: Parser<I, O>> Parser<I, Vec<O>> for Repeat<P> {
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<Vec<O>, Error<I>>)
    where
        Self: Sized,
    {
        let mut outputs = vec![];
        let mut p = stream.position();

        loop {
            match self.0.parse_impl(stream, errors) {
                (np, Ok(o)) => {
                    p = np;
                    outputs.push(o);
                }
                _ => break,
            }
        }

        (p, Ok(outputs))
    }
}

/// For constructing Repeat.
pub fn repeat<P>(p: P) -> Repeat<P> {
    Repeat(p)
}
