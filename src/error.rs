/// # Parcos Error
///  Gets returned when parser fails.
#[derive(Debug)]
pub enum Error<I> {
    /// Occurs when expected something but got something else.
    Unexpected(usize, Vec<I>, Option<I>),
}

impl<I> Error<I> {
    fn position(&self) -> usize {
        match self {
            Error::Unexpected(p, _, _) => *p,
        }
    }

    pub fn merge(self, other: Self) -> Self {
        if self.position() > other.position() {
            self
        } else if self.position() < other.position() {
            other
        } else {
            match (self, other) {
                (Error::Unexpected(p, mut e1, f1), Error::Unexpected(_, mut e2, f2)) => {
                    e1.append(&mut e2);
                    Error::Unexpected(p, e1, f1.or(f2))
                }
            }
        }
    }
}
