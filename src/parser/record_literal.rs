use crate::parser::parser_utils::ParseResult;
use crate::parser::expression::{Expression, parse_expression};
use crate::lexer::Token;


#[derive(Debug)]
#[allow(dead_code)]
pub struct Record {
    fields: Vec<(String, Expression)>,
}

pub fn parse_record_literal(tokens: &[Token]) -> ParseResult<Record> {
    let mut idx: usize = 0;
    let mut record = Record { fields: Vec::new() };

    if tokens.get(idx) != Some(&Token::CurlyBracket('{')) {
        return ParseResult::Failure("Expected curly bracket at the beginning of record literal".to_string(), 0);
    }

    idx += 1;
    loop {
        if tokens.get(idx) == Some(&Token::CurlyBracket('}')) {
            break;
        }

        if let Token::Identifier(s) = &tokens[idx] {
            idx += 1;
            if tokens.get(idx) != Some(&Token::SingleLeftArrow) {
                return ParseResult::Failure("Expected left arrow after identifier".to_string(), 0);
            }
            idx += 1;
            match parse_expression(&tokens[idx..]) {
                ParseResult::Failure(msg, depth) => {
                    return ParseResult::Failure(msg, depth+1);
                }
                ParseResult::Success(expression, new_idx) => {
                    idx += new_idx;
                    record.fields.push((s.clone(), expression));
                }
            }
        } else {
            return ParseResult::Failure("Expected identifier at the beginning of field".to_string(), 0);
        }

        if tokens.get(idx) == Some(&Token::Comma) {
            idx += 1;
        }
    }

    idx += 1; // skip over final curly bracket

    ParseResult::Success(record, idx)
}
