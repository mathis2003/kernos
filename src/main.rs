mod lexer;
use lexer::{lex, Token};

mod parser;
use parser::{parse_function_literal, parse_record_literal, ParseResult};

//const LITERALS_STRING: &'static str = "11 abc()[]{}";
/*const TEST_CODE_STRING: &'static str = "program.add({
    c <- { x <- 2, y <- 3},
    nodes: [],
    render = \\center, nodes -> {
        doStuff();
        return this;
    },
    
    interpret = \\langeElement -> {
        doStuff();
        return something;
    },

}");*/

/*const TEST_FUNCTION_STRING: &'static str = "\\a, b -> {
    f <- \\-> { return 1; };
    return 2;
}";*/

//const TEST_RECORD_STRING: &'static str = "{ a <- 1, b <- 2, f <- \\-> { return 1; }}";
// const identifiers_string: &'static str = "abc def";
// const numbers_string: &'static str = "123 456";

fn main() {
    /*let tokens: Vec<Token> = lex(TEST_CODE_STRING);
    for token in tokens {
        println!("{:?}", token);
    }*/

    /*println!("\n\nFunction tokens:\n\n");

    let function_tokens: Vec<Token> = lex(TEST_FUNCTION_STRING);
    for token in &function_tokens {
        println!("{:?}", token);
    }

    println!("\n\nFunction literal AST:\n\n");

    let (function_literal, _) = parse_function_literal(&function_tokens);

    println!("{:?}", function_literal);*/

    let record_file_contents = std::fs::read_to_string("test_programs/test_record.txt").expect("Should have been able to read the file");

    println!("\n\nRecord tokens:\n\n");
    let record_tokens: Vec<Token> = lex(record_file_contents.as_str());
    for token in &record_tokens {
        println!("{:?}", token);
    }

    println!("\n\nRecord literal AST:\n\n");
    match parse_record_literal(&record_tokens) {
        ParseResult::Success(record_literal, _) => {
            println!("{:?}", record_literal);
        }
        ParseResult::Failure(e, _) => {
            println!("{:?}", e);
        }
    }

}
