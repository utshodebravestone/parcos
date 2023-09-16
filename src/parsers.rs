pub fn exact(expected: &'static str) -> impl Fn(&str) -> Result<(&str, &str), String> {
    move |input| match input.get(..expected.len()) {
        Some(it) if it == expected => Ok((&input[expected.len()..], &input[..expected.len()])),
        _ => Err(format!("expected `{expected}`, got unexpected `{input}`")),
    }
}

pub fn any_of(
    parsers: Vec<impl Fn(&str) -> Result<(&str, &str), String>>,
) -> impl Fn(&str) -> Result<(&str, &str), String> {
    move |input| {
        let mut errors = vec![];
        for parser in &parsers {
            match parser(input) {
                Ok(ok) => return Ok(ok),
                Err(err) => errors.push(err),
            }
        }
        Err(format!("couldn't match any of:\n{}", errors.join("\n")))
    }
}

pub fn zero_or_more(
    parser: impl Fn(&str) -> Result<(&str, &str), String>,
) -> impl Fn(&str) -> Result<(&str, &str), String> {
    move |input| {
        let mut consumed_till = 0;
        while let Ok(ok) = parser(&input[consumed_till..]) {
            consumed_till += ok.1.len();
        }
        Ok((&input[consumed_till..], &input[..consumed_till]))
    }
}

pub fn whitespace(input: &str) -> Result<(&str, &str), String> {
    let parse_whitespace = exact(" ");
    let parse_tab = exact("\t");
    let parse_new_line = exact("\n");
    let parse_any_whitespace = any_of(vec![parse_whitespace, parse_tab, parse_new_line]);

    parse_any_whitespace(input)
}

pub fn whitespaces(input: &str) -> Result<(&str, &str), String> {
    let parse_whitespaces = zero_or_more(whitespace);
    parse_whitespaces(input)
}

pub fn identifier(input: &str) -> Result<(&str, &str), String> {
    let mut matched_till = 0;
    let mut chars = input.chars();

    match chars.next() {
        Some(it) if it.is_alphabetic() || it == '_' => matched_till += 1,
        _ => return Err("expected `IDENTIFIER`, got unexpected end of input".into()),
    }

    while let Some(it) = chars.next() {
        if it.is_alphanumeric() || it == '_' {
            matched_till += 1;
        } else {
            break;
        }
    }

    Ok((&input[matched_till..], &input[..matched_till]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_exact() {
        let parse_foo = exact("foo");

        assert_eq!(Ok(("bar", "foo")), parse_foo("foobar"));
        assert_eq!(Ok(("", "foo")), parse_foo("foo"));
        assert!(parse_foo("bar").is_err());
        assert!(parse_foo("").is_err());
    }

    #[test]
    fn parse_any_of() {
        let parse_plus = exact("+");
        let parse_minus = exact("-");
        let parse_plus_or_minus = any_of(vec![parse_plus, parse_minus]);

        assert_eq!(Ok(("", "+",)), parse_plus_or_minus("+"));
        assert_eq!(Ok(("", "-",)), parse_plus_or_minus("-"));
        assert_eq!(Ok(("-", "+")), parse_plus_or_minus("+-"));
        assert_eq!(Ok(("+", "-")), parse_plus_or_minus("-+"));

        assert!(parse_plus_or_minus("").is_err());
        assert!(parse_plus_or_minus("foo bar").is_err());
    }

    #[test]
    fn parse_whitespace() {
        assert_eq!(Ok(("", " ")), whitespace(" "));
        assert_eq!(Ok(("", "\t")), whitespace("\t"));
        assert_eq!(Ok(("", "\n")), whitespace("\n"));
        assert_eq!(Ok((" \n", "\t")), whitespace("\t \n"));
        assert!(whitespace("foo bar").is_err());
        assert!(whitespace("").is_err());
    }

    #[test]
    fn parse_whitespaces() {
        assert_eq!(Ok(("", " ")), whitespaces(" "));
        assert_eq!(Ok(("", "\t")), whitespaces("\t"));
        assert_eq!(Ok(("", "\n")), whitespaces("\n"));
        assert_eq!(Ok(("", "\t \n")), whitespaces("\t \n"));
        assert_eq!(Ok(("foo bar", "")), whitespaces("foo bar"));
        assert_eq!(Ok(("", "")), whitespaces(""));
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(Ok((" bar", "foo".into())), identifier("foo bar"));
        assert_eq!(Ok(("", "foobar".into())), identifier("foobar"));
        assert_eq!(Ok(("", "_foobar".into())), identifier("_foobar"));
        assert_eq!(Ok(("", "foobar0".into())), identifier("foobar0"));
        assert_eq!(Ok(("", "foo_bar_1".into())), identifier("foo_bar_1"));
        assert!(identifier("!foobar").is_err());
        assert!(identifier("").is_err());
    }
}
