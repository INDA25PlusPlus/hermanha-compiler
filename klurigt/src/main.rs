mod lexer;
use crate::lexer::Lexer;



fn main() {
    let lexer = Lexer::new("make a: number be 0 STOP
make b: number be 1 STOP
make depth: number be 5 STOP
make count: number be 0 STOP
make fib: number be 0 STOP
keeponswimming(count tinyerthan depth){
    fib be a+b STOP
    a be b STOP
    b be fib STOP
    } STOPSWIMMING
SCREAM(fib)QUIET".to_string());
    for token in lexer {
        println!("{:?}", token);
    }
}
