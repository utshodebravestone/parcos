use crate::{
    error::Error,
    stream::{Stream, Streamable},
};

/// # Parcos Parser
/// All of parcos combinators implements this trait.
pub trait Parser<I, O> {
    /// This is the method any combinator implements.
    /// It's (typically) a good idea to not call it from a parser.
    fn parse_impl<Stream: Streamable<I>>(
        &self,
        stream: &mut Stream,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<O, Error<I>>)
    where
        Self: Sized;

    /// This method tries to parse via parse_impl and returns errors if can't parse.
    /// When it returns error the output always will be none and vice-versa.
    fn parse_or_recover<Iter: Iterator<Item = I>>(&self, iter: Iter) -> (Option<O>, Vec<Error<I>>)
    where
        Self: Sized,
    {
        let mut errors = vec![];
        match self.parse_impl(&mut Stream::new(iter), &mut errors).1 {
            Ok(output) => (Some(output), errors),
            Err(error) => {
                errors.push(error);
                (None, errors)
            }
        }
    }

    /// This method parses what was intended or returns errors.
    fn parse<Iter: Iterator<Item = I>>(&self, iter: Iter) -> Result<O, Vec<Error<I>>>
    where
        Self: Sized,
    {
        let (output, errors) = self.parse_or_recover(iter);
        if errors.is_empty() {
            // output should be some variant since we're accessing it only if errors is empty
            Ok(output.unwrap())
        } else {
            Err(errors)
        }
    }
}
