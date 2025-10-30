mod ast;
mod lexer;
mod parser;
mod codegen;

use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let input = match fs::read_to_string("fib.dumb") {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Error: Could not read input file: {}", e);
            return;
        }
    };

    let lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer.collect();
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    let rust_code = codegen::generate_rust(&program);

    if let Err(e) = fs::write("output.rs", rust_code) {
        eprintln!("Could not write output file: {}", e);
        return;
    }
}
