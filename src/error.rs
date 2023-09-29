/// # Parcos Error
///  Gets returned when parser fails.
#[derive(Debug)]
pub enum Error<I> {
    /// Occurs when expected something but got something else.
    Unexpected(usize, I, Option<I>),
}
