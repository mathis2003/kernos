use crate::parser::parser_utils::ParseResult;
use crate::parser::function_literal::{ parse_function_literal, FunctionLiteral };
use crate::parser::record_literal::{ parse_record_literal, Record };
use crate::lexer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i64),
    RecordLiteral(Record),
    FunctionLiteral(FunctionLiteral),
}


pub fn parse_expression(tokens: &[Token]) -> ParseResult<Expression> {
    let mut idx: usize = 0;
    let expression: Expression;

    match &tokens[idx] {
        Token::Identifier(s) => {
            expression = Expression::Identifier(s.clone());
            idx += 1;
        }
        Token::Num(n) => {
            expression = Expression::NumberLiteral(*n);
            idx += 1;
        }
        Token::BackSlash => {
            match parse_function_literal(&tokens[idx..]) {
                ParseResult::Failure(msg, depth) => {
                    return ParseResult::Failure(msg, depth+1);
                }
                ParseResult::Success(function_literal, new_idx) => {
                    idx += new_idx;
                    expression = Expression::FunctionLiteral(function_literal);
                }
            }
        }
        Token::CurlyBracket('{') => {
            match parse_record_literal(&tokens[idx..]) {
                ParseResult::Failure(msg, depth) => {
                    return ParseResult::Failure(msg, depth+1);
                }
                ParseResult::Success(record_literal, new_idx) => {
                    idx += new_idx;
                    expression = Expression::RecordLiteral(record_literal);
                }
            }
        }
        _ => { return ParseResult::Failure("Expected identifier, number literal, or function literal at the beginning of expression".to_string(), 0); }
    }

    ParseResult::Success(expression, idx)
}
