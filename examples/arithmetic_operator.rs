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

    let mut src = "+ - * / wat".split(" ");

    println!("{:#?}", parser.parse(&mut src));
    println!("{:#?}", parser.parse(&mut src));
    println!("{:#?}", parser.parse(&mut src));
    println!("{:#?}", parser.parse(&mut src));
    println!("{:#?}", parser.parse(&mut src));
}
