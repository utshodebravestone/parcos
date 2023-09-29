use crate::{combinators::just, parser::Parser};

#[test]
fn test_just_parser() {
    let foo_parser = just("foo");
    let parsed = foo_parser.parse(vec!["foo"].into_iter());

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), "foo");
}

#[test]
fn test_just_parser_failure() {
    let foo_parser = just("foo");
    let parsed = foo_parser.parse(vec!["bar"].into_iter());

    assert!(parsed.is_err());
}
