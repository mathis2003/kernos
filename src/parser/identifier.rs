use crate::parser::parser_utils::{ParseResult, eat_whitespace};

#[derive(Debug)]
pub struct Identifier(pub String);

pub fn parse_identifier(chars: &[char]) -> ParseResult<Identifier> {
    let mut idx: usize = 0;
    let identifier: Identifier;

    idx += eat_whitespace(&chars[idx..]);

    match &chars[idx] {
        'a'..='z' | 'A'..='Z' | '_' => {
            let mut id = String::new();
            while idx < chars.len() {
                match chars[idx] {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        id.push(chars[idx]);
                        idx += 1;
                    }
                    _ => { break; }
                }
            }
            identifier = Identifier(id);
        }
        _ => { return ParseResult::Failure("Expected identifier at the beginning of expression".to_string(), 0); }
    }

    idx += eat_whitespace(&chars[idx..]);

    ParseResult::Success(identifier, idx)
}