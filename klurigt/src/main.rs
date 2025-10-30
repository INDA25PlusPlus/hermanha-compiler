mod ast;
mod lexer;
mod parser;
mod codegen;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let input = "make a: number be 0 STOP
        make b: number be 1 STOP
        make depth: number be 5 STOP
        make count: number be 0 STOP
        make fib: number be 0 STOP
        keeponswimming(count tinyerthan depth){
            fib be a+b STOP
            a be b STOP
            b be fib STOP
            count be count +1 STOP
            } STOPSWIMMING
        SCREAM(fib)QUIET";

    let lexer = Lexer::new(input.to_string());
    let tokens: Vec<_> = lexer.collect();

    let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(program) => {
            println!("{}", codegen::generate_rust(&program));
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
