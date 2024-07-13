use crate::parser::parser_utils::{ParseResult, eat_whitespace};

#[derive(Debug)]
pub struct Number(i64);

pub fn parse_number(chars: &[char]) -> ParseResult<Number> {
    let mut idx: usize = 0;
    let number: Number;

    idx += eat_whitespace(&chars[idx..]);

    match &chars[idx] {
        '0'..='9' => {
            let mut num = 0;
            while idx < chars.len() {
                match chars[idx] {
                    '0' ..= '9' => { 
                        num = num * 10 + (chars[idx] as i64 - '0' as i64);
                        idx += 1;
                    }
                    _ => { break; }
                }
            }
            number = Number(num);
        }
        _ => { return ParseResult::Failure("Expected number literal at the beginning of expression".to_string(), 0); }
    }

    idx += eat_whitespace(&chars[idx..]);

    ParseResult::Success(number, idx)
}