mod lexer;

use lexer::{lex, Token};

const LITERALS_STRING: &'static str = "11 abc()[]{}";
// const identifiers_string: &'static str = "abc def";
// const numbers_string: &'static str = "123 456";

fn main() {
    let tokens: Vec<Token> = lex(LITERALS_STRING);
    for token in tokens {
        println!("{:?}", token);
    }
}
