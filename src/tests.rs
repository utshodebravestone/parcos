use crate::{combinators::just, parser::Parser};

#[test]
fn test_just_parser() {
    let foo_parser = just("foo");
    let parsed = foo_parser.parse(vec!["foo"].into_iter());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "foo");

    let slash_parser = just('/');
    let parsed = slash_parser.parse("/foo".chars());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), '/');
}

#[test]
fn test_just_parser_failure() {
    let foo_parser = just("foo");
    let parsed: Result<&str, Vec<crate::error::Error<&str>>> =
        foo_parser.parse(vec!["bar"].into_iter());

    assert!(parsed.is_err());
}

#[test]
fn test_to_parser() {
    let foo_parser = just("foo").to("bar");
    let parsed = foo_parser.parse(vec!["foo"].into_iter());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "bar");

    let slash_parser = just('/').to("Slash");
    let parsed = slash_parser.parse("/foo".chars());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "Slash");
}

#[test]
fn test_or_parser() {
    let foo_parser = just("foo");
    let bar_parser = just("bar");
    let foo_bar_parser = foo_parser.or(bar_parser);
    let parsed = foo_bar_parser.parse(vec!["foo"].into_iter());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "foo");

    let bang_parser = just('!').to("Bang");
    let slash_parser = just('/').to("Slash");
    let bang_or_slash_parser = bang_parser.or(slash_parser);
    let parsed = bang_or_slash_parser.parse("/".chars());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "Slash");
}
