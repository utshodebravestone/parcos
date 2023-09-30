use std::marker::PhantomData;

use crate::{error::Error, parser::Parser, stream::Streamable};

/// # Map
/// For mapping output from one type to other (O -> N) via a function (F).
pub struct Map<P, F, O>(P, F, PhantomData<O>);

impl<I, O, N, P: Parser<I, O>, F: Fn(O) -> N> Parser<I, N> for Map<P, F, O> {
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<N, Error<I>>)
    where
        Self: Sized,
    {
        let (p, res) = self.0.parse_impl(stream, errors);
        (p, res.map(&self.1))
    }
}

/// For constructing Map.
pub fn map<I, O, P: Parser<I, O>, F>(p: P, f: F) -> Map<P, F, O> {
    Map(p, f, PhantomData)
}
