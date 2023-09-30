use parcos::{
    combinators::{just, pred},
    error::Error,
    parser::Parser,
};

#[derive(Debug, Clone)]
enum Operator {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
}

fn main() {
    let parser = pred(|x: &char| x.is_ascii_digit())
        .to(Operator::Number)
        .or(just('+').to(Operator::Plus))
        .or(just('-').to(Operator::Minus))
        .or(just('*').to(Operator::Star))
        .or(just('/').to(Operator::Slash));

    let mut src = "1+2*3/4".chars();

    loop {
        match parser.parse(&mut src) {
            Ok(o) => println!("{o:?}"),
            Err(es) => {
                for e in es {
                    match e {
                        Error::Unexpected(p, e, f) => {
                            eprintln!("unexpected token. expected: {e:?}, found: {f:?} in {p}");
                        }
                    }
                }
                break;
            }
        }
    }
}
