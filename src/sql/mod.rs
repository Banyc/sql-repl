use chumsky::prelude::*;

use self::{
    lexer::{lexer, Literal, Token},
    stat::{parser, Stat},
};

pub mod expr;
pub mod lexer;
pub mod stat;

pub fn parse(src: &str) -> Option<S> {
    let lexer = lexer();
    let Some(tokens) = lexer
        .parse(src)
        .into_result()
        .map_err(|e| eprint!("{e:?}"))
        .ok()
    else {
        return None;
    };
    let parser = parser();
    let Some(ast) = parser
        .parse(&tokens)
        .into_result()
        .map_err(|e| eprintln!("{e:?}"))
        .ok()
    else {
        return None;
    };
    Some(ast)
}

#[derive(Debug, Clone, PartialEq)]
pub struct S {
    pub statements: Vec<Stat>,
}

fn string_token<'a>() -> impl Parser<'a, &'a [Token], String, extra::Err<Rich<'a, Token>>> + Clone {
    select_ref! { Token::Literal(Literal::String(name)) => name.clone() }
}

fn variable_token<'a>() -> impl Parser<'a, &'a [Token], String, extra::Err<Rich<'a, Token>>> + Clone
{
    select_ref! { Token::Variable(name) => name.clone() }
}