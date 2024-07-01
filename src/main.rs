mod lexer;

use lexer::{lex, Token};

//const LITERALS_STRING: &'static str = "11 abc()[]{}";
const TEST_CODE_STRING: &'static str = "struct LanguageElement {
    c: Center,
    nodes: [],
    render = \\center, nodes -> {
        doStuff();
        return this;
    },
    
    interpret = \\langeElement -> {
        doStuff();
        return something;
    },

}";
// const identifiers_string: &'static str = "abc def";
// const numbers_string: &'static str = "123 456";

fn main() {
    let tokens: Vec<Token> = lex(TEST_CODE_STRING);
    for token in tokens {
        println!("{:?}", token);
    }
}
