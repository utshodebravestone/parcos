use crate::*;

/// Parses a single character (passed via target_char)
///
///
/// # Example
///
/// ```
/// use parcos::ParsedObject;
/// use parcos::combinators::character;
///
/// fn it_works() {
///     assert_eq!(character('f')("foo"), Ok(("oo".into(), ParsedObject::Char('f'))));
/// }
/// ```
///
pub fn character(target_char: char) -> Parser {
    let p = move |source: &str| match source.chars().nth(0) {
        Some(char) => match char == target_char {
            true => Ok((source[1..].into(), ParsedObject::Char(char))),
            false => Err(ParseError::Unexpected(target_char.into(), char.into(), 0)),
        },
        None => Err(ParseError::NotEnoughInput(target_char.into(), 0)),
    };
    Box::new(p)
}

/// Parses an exact string (passed via target_str)
///
///
/// # Example
///
/// ```
/// use parcos::ParsedObject;
/// use parcos::combinators::exact;
///
/// fn it_works() {
///     assert_eq!(
///         exact("foo")("foo"),
///         Ok(("".into(), ParsedObject::String("foo".into())))
///     );
/// }
/// ```
///
pub fn exact(target_str: &'static str) -> Parser {
    let p = move |source: &str| {
        if &source[0..target_str.len()] == target_str {
            Ok((
                source[source.len()..].into(),
                ParsedObject::String(source[0..target_str.len()].into()),
            ))
        } else {
            Err(ParseError::Unexpected(
                target_str.into(),
                source[0..target_str.len()].into(),
                0,
            ))
        }
    };
    Box::new(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert_eq!(
            character('f')("foo"),
            Ok(("oo".into(), ParsedObject::Char('f')))
        );
        assert_eq!(
            character('f')("bar"),
            Err(ParseError::Unexpected("f".into(), "b".into(), 0))
        );
        assert_eq!(
            character('f')(""),
            Err(ParseError::NotEnoughInput("f".into(), 0))
        );
    }

    #[test]
    fn test_exact() {
        assert_eq!(
            exact("foo")("foo"),
            Ok(("".into(), ParsedObject::String("foo".into())))
        );
    }
}
