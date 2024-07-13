//mod lexer;
//use lexer::{lex, Token};

mod parser;
use crate::parser::record_literal::{ parse_record_literal };
use crate::parser::parser_utils::ParseResult;


fn main() {
    let record_file_contents = std::fs::read_to_string("test_programs/test_record.txt").expect("Should have been able to read the file");

    /*println!("\n\nRecord tokens:\n\n");
    let record_tokens: Vec<Token> = lex(record_file_contents.as_str());
    for token in &record_tokens {
        println!("{:?}", token);
    }*/



    println!("\n\nRecord literal AST:\n\n");
    match parse_record_literal(record_file_contents.chars().collect::<Vec<char>>().as_slice()) {
        ParseResult::Success(record_literal, _) => {
            println!("{:?}", record_literal);
        }
        ParseResult::Failure(e, _) => {
            println!("{:?}", e);
        }
    }

}
