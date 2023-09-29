use std::iter::Peekable;

/// # Parcos Input Streams
/// Types that implements this trait can be used as input for parser.
pub trait Streamable<I> {
    fn peek(&mut self) -> Option<&I>;

    fn next(&mut self) -> Option<I>;

    fn position(&self) -> usize;
}

/// This makes an iterator parser input (Streamable).
pub struct Stream<Iter: Iterator>(Peekable<Iter>, usize);

impl<Iter: Iterator> Stream<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self(iter.peekable(), 0)
    }
}

impl<Iter: Iterator> Streamable<Iter::Item> for Stream<Iter> {
    fn peek(&mut self) -> Option<&Iter::Item> {
        self.0.peek()
    }

    fn next(&mut self) -> Option<Iter::Item> {
        self.1 += 1;
        self.0.next()
    }

    fn position(&self) -> usize {
        self.1
    }
}
