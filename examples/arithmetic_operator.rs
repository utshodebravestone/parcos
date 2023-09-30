use std::borrow::BorrowMut;

use parcos::{
    combinators::{just, pred},
    parser::Parser,
};

#[derive(Debug, Clone)]
enum Node {
    Number(u32),
    Plus,
    Minus,
    Star,
    Slash,
}

fn main() {
    let parser = pred(|x: &char| x.is_digit(10))
        .map(|o| Node::Number(o.to_digit(10).unwrap()))
        .or(just('+').to(Node::Plus))
        .or(just('-').to(Node::Minus))
        .or(just('*').to(Node::Star))
        .or(just('/').to(Node::Slash));

    let mut src = "1+2*3/4".chars();

    while let Ok(n) = parser.parse(src.borrow_mut()) {
        println!("{n:#?}");
    }
}
