use crate::parser::parser_utils::{ParseResult, eat_whitespace};
use crate::parser::expression::{Expression, parse_expression};
use crate::parser::identifier::{parse_identifier, Identifier};
//use crate::parser::number::{parse_number, Number};


#[derive(Debug)]
#[allow(dead_code)]
pub struct Record {
    fields: Vec<(String, Expression)>,
}

pub fn parse_record_literal(chars: &[char]) -> ParseResult<Record> {
    let mut idx: usize = 0;
    let mut record = Record { fields: Vec::new() };

    idx += eat_whitespace(&chars[idx..]);

    if chars.get(idx) != Some(&'{') { //TODO: fix cast
        return ParseResult::Failure("Expected curly bracket at the beginning of record literal".to_string(), 0);
    }

    idx += 1; // skip over initial curly bracket

    loop {
        idx += eat_whitespace(&chars[idx..]);
        if chars.get(idx) == Some(&'}') {
            break;
        }

        match parse_identifier(&chars[idx..]) {
            ParseResult::Success(Identifier(s), offset) => {
                idx += offset;
                if &chars[idx..idx+2] != "<-".to_string().chars().collect::<Vec<char>>().as_slice() {
                    return ParseResult::Failure("Expected left arrow after identifier".to_string(), 0);
                }
                idx += 2; // skip over left arrow
                match parse_expression(&chars[idx..]) {
                    ParseResult::Failure(msg, depth) => {
                        return ParseResult::Failure(msg, depth+1);
                    }
                    ParseResult::Success(expression, new_idx) => {
                        idx += new_idx;
                        record.fields.push((s.clone(), expression));
                    }
                }
            }
            ParseResult::Failure(msg, depth) => {
                return ParseResult::Failure(msg, depth+1);
            }
        }

        if chars.get(idx) == Some(&',') {
            idx += 1;
        }
    }

    idx += 1; // skip over final curly bracket

    idx += eat_whitespace(&chars[idx..]);

    ParseResult::Success(record, idx)
}
