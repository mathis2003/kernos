use crate::parser::parser_utils::{ParseResult, eat_whitespace};
use crate::parser::function_literal::{ parse_function_literal, FunctionLiteral };
use crate::parser::record_literal::{ parse_record_literal, Record };
use crate::parser::identifier::{ parse_identifier, Identifier };
use crate::parser::number::{ parse_number, Number };

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expression {
    Identifier(Identifier),
    NumberLiteral(Number),
    Record(Record),
    FunctionLiteral(FunctionLiteral),
}

pub fn parse_expression(chars: &[char]) -> ParseResult<Expression> {

    let mut parse_error = ParseResult::Failure("unable to parse expression".to_string(), 0);
    let mut parse_error_depth = 0;

    let idx = eat_whitespace(chars);

    match parse_identifier(&chars[idx..]) {
        ParseResult::Success(identifier, offset) => {
            return ParseResult::Success(Expression::Identifier(identifier), idx + offset);
        }
        ParseResult::Failure(msg, depth) => {
            if depth >= parse_error_depth {
                parse_error = ParseResult::Failure(msg, depth+1);
                parse_error_depth = depth;
            }
        }
    }

    match parse_number(&chars[idx..]) {
        ParseResult::Success(number, offset) => {
            return ParseResult::Success(Expression::NumberLiteral(number), idx + offset);
        }
        ParseResult::Failure(msg, depth) => {
            if depth >= parse_error_depth {
                parse_error = ParseResult::Failure(msg, depth+1);
                parse_error_depth = depth;
            }
        }
    }

    match parse_function_literal(&chars[idx..]) {
        ParseResult::Success(function_literal, offset) => {
            return ParseResult::Success(Expression::FunctionLiteral(function_literal), idx + offset);
        }
        ParseResult::Failure(msg, depth) => {
            if depth >= parse_error_depth {
                parse_error = ParseResult::Failure(msg, depth+1);
                parse_error_depth = depth;
            }
        }
    }

    match parse_record_literal(&chars[idx..]) {
        ParseResult::Success(record_literal, offset) => {
            return ParseResult::Success(Expression::Record(record_literal), idx + offset);
        }
        ParseResult::Failure(msg, depth) => {
            if depth >= parse_error_depth {
                parse_error = ParseResult::Failure(msg, depth+1);
                //parse_error_depth = depth;
            }
        }
    }

    parse_error

}
