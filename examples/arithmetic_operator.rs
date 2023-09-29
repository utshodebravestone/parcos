use parcos::{combinators::just, parser::Parser};

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

fn main() {
    let parser = just("+")
        .to(Operator::Plus)
        .or(just("-").to(Operator::Minus))
        .or(just("*").to(Operator::Star))
        .or(just("/").to(Operator::Slash));

    println!("{:#?}", parser.parse("+".split(" ")));
    println!("{:#?}", parser.parse("-".split(" ")));
    println!("{:#?}", parser.parse("*".split(" ")));
    println!("{:#?}", parser.parse("/".split(" ")));
    println!("{:#?}", parser.parse("wat".split(" ")));
}
