#[cfg(test)]
mod tests {
    use crate::*;

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
    fn parse_zero_or_more() {
        let parse_plus = exact("+");
        let parse_pluses = zero_or_more(parse_plus);

        assert_eq!(Ok(("", "+")), parse_pluses("+"));
        assert_eq!(Ok(("", "++")), parse_pluses("++"));
        assert_eq!(Ok(("", "")), parse_pluses(""));
        assert_eq!(Ok(("foobar", "")), parse_pluses("foobar"));
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
    fn parse_digit() {
        assert_eq!(Ok(("6", "2".into())), digit("26"));
        assert_eq!(Ok(("6s", "2".into())), digit("26s"));
        assert!(digit(".26").is_err());
        assert!(digit("").is_err());
    }

    #[test]
    fn parse_digits() {
        assert_eq!(Ok(("", "26".into())), digits("26"));
        assert_eq!(Ok(("s", "26".into())), digits("26s"));
        assert!(digits(".26").is_err());
        assert!(digits("").is_err());
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
