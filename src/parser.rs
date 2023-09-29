use crate::{
    combinators::{or, to, Or, To},
    error::Error,
    stream::{Stream, Streamable},
};

/// # Parcos Parser
/// For making a type Parser.
pub trait Parser<I, O> {
    /// For other combinators to implement.
    fn parse_impl<S: Streamable<I>>(
        &self,
        stream: &mut S,
        errors: &mut Vec<Error<I>>,
    ) -> (usize, Result<O, Error<I>>)
    where
        Self: Sized;

    /// Tries to parse via parse_impl and returns errors if can't parse.
    /// When it returns error the output always will be none and vice-versa.
    fn parse_or_recover<Iter: Iterator<Item = I>>(&self, iter: Iter) -> (Option<O>, Vec<Error<I>>)
    where
        Self: Sized,
    {
        let mut errors = vec![];
        match self.parse_impl(&mut Stream::new(iter), &mut errors).1 {
            Ok(o) => (Some(o), errors),
            Err(e) => {
                errors.push(e);
                (None, errors)
            }
        }
    }

    /// Parses what was intended or returns errors.
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

    /// For converting output type (O to N)
    /// ```
    /// use parcos::{parser::Parser, combinators::just};
    ///
    /// let slash_parser = just('/').to("Slash");
    /// let parsed = slash_parser.parse("/foo".chars());
    ///
    /// assert!(parsed.is_ok());
    /// assert_eq!(parsed.unwrap(), "Slash");
    /// ```
    fn to<N: Clone>(self, x: N) -> To<Self, N, O>
    where
        Self: Sized,
    {
        to(self, x)
    }

    /// For choosing between two parser (P1 and P2)
    /// ```
    /// use parcos::{parser::Parser, combinators::just};
    ///
    /// let bang_parser = just('!').to("Bang");
    /// let slash_parser = just('/').to("Slash");
    /// let bang_or_slash_parser = bang_parser.or(slash_parser);
    /// let parsed = bang_or_slash_parser.parse("/".chars());
    ///
    /// assert!(parsed.is_ok());
    /// assert_eq!(parsed.unwrap(), "Slash");
    /// ```
    fn or<P: Parser<I, O>>(self, other: P) -> Or<Self, P>
    where
        Self: Sized,
    {
        or(self, other)
    }
}
