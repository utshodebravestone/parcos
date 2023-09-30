use std::marker::PhantomData;

use crate::{error::Error, parser::Parser, stream::Streamable};

/// # To
/// For converting output type (O -> N).
pub struct To<P, N, O>(P, N, PhantomData<O>);

impl<I, O, P: Parser<I, O>, N: Clone> Parser<I, N> for To<P, N, O> {
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<N, Error<I>>)
    where
        Self: Sized,
    {
        let (p, res) = self.0.parse_impl(stream, errors);
        (p, res.map(|_| self.1.clone()))
    }
}

/// For constructing To.
pub fn to<P, N, O>(parser: P, new_output: N) -> To<P, N, O> {
    To(parser, new_output, PhantomData)
}
