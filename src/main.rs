mod ast;
mod token;
mod lexer;
mod parser;
mod interpreter;
mod engine;

use std::fs;
use crate::{lexer::Lexer, parser::Parser, engine::Engine};

fn main() {
    let src = fs::read_to_string("test.num").unwrap();

    let mut lexer = Lexer::new(&src);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(ast) => {
            let mut engine = Engine::new();
            engine.run(ast);
        }
        Err(e) => {
            println!("Syntax error: {e}");
        }
    }
}