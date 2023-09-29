/// # Parcos Error
///  This gets returned when parser fails.
#[derive(Debug)]
pub enum Error<I> {
    /// Occurs when expected something but couldn't get that.
    Unexpected(usize, I, Option<I>),
}
