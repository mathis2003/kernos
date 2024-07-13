use crate::parser::parser_utils::{ParseResult, eat_whitespace};

#[derive(Debug)]
pub struct Keyword(String);

const KEYWORDS: &[&str] = &["if", "then", "else", "struct", "variant", "return"];

pub fn parse_keyword(chars: &[char]) -> ParseResult<Keyword> {
    let mut idx: usize = 0;
    let mut id = String::new();

    idx += eat_whitespace(&chars[idx..]);

    match chars.get(idx) {
        Some('a'..='z') | Some('A'..='Z') | Some('_') => {
            while idx < chars.len() {
                match chars[idx] {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        id.push(chars[idx]);
                        idx += 1;
                    }
                    _ => { break; }
                }
            }
        }
        _ => { return ParseResult::Failure("Expected keyword to start with alphabet symbol".to_string(), 0); }
    }

    idx += eat_whitespace(&chars[idx..]);

    if KEYWORDS.contains(&id.as_str()) {
        ParseResult::Success(Keyword(id), idx)
    } else {
        ParseResult::Failure("Expected keyword to start with alphabet symbol".to_string(), 0)
    }
}
