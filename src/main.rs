mod lexer;
use lexer::{lex, Token};

mod parser;
use parser::{parse_function_literal};

//const LITERALS_STRING: &'static str = "11 abc()[]{}";
/*const TEST_CODE_STRING: &'static str = "struct LanguageElement {
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

}";*/

const TEST_FUNCTION_STRING: &'static str = "\\a, b -> {
    f <- \\-> { return 1; };
    return 2;
}";
// const identifiers_string: &'static str = "abc def";
// const numbers_string: &'static str = "123 456";

fn main() {
    /*let tokens: Vec<Token> = lex(TEST_CODE_STRING);
    for token in tokens {
        println!("{:?}", token);
    }*/

    println!("\n\nFunction tokens:\n\n");

    let function_tokens: Vec<Token> = lex(TEST_FUNCTION_STRING);
    for token in &function_tokens {
        println!("{:?}", token);
    }

    println!("\n\nFunction literal AST:\n\n");

    let (function_literal, _) = parse_function_literal(&function_tokens);

    println!("{:?}", function_literal);

}
