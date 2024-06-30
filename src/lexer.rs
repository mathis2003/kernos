
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Parenthesis(char),
    CurlyBracket(char),
    SquareBracket(char),
    Num(i64),
    Str(String),
}

pub fn lex(code_str: &str) -> Vec<Token> {

    let mut idx: usize = 0;
    let mut result_tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = code_str.chars().collect();

    while idx < chars.len() {
        match chars[idx] {
            'a' ..= 'z' | 'A'..='Z' | '_' => {
                result_tokens.push(parse_identifier(&chars, &mut idx));
            }
            '0' ..= '9' => {
                result_tokens.push(parse_number(&chars, &mut idx));
            }
            ' ' | '\n' | '\t' => {
                idx += 1;
            }

            '(' | ')' => {
                result_tokens.push(Token::Parenthesis(chars[idx]));
                idx += 1;
            }

            '{' | '}' => {
                result_tokens.push(Token::CurlyBracket(chars[idx]));
                idx += 1;
            }

            '[' | ']' => {
                result_tokens.push(Token::SquareBracket(chars[idx]));
                idx += 1;
            }

            _ => {
                idx += 1;
            }

        }

    }

    return result_tokens;
}

fn parse_identifier(chars: &[char], idx: &mut usize) -> Token {
    let mut identifier = String::new();
    while *idx < chars.len() {
        match chars[*idx] {
            'a' ..= 'z' | 'A'..='Z' | '_' | '0' ..= '9' => { 
                identifier.push(chars[*idx]);
                *idx += 1;
            },
            _ => break,
        }
    }

    return Token::Identifier(identifier);
}

fn parse_number(chars: &[char], idx: &mut usize) -> Token {
    let mut num = 0;
    while *idx < chars.len() {
        match chars[*idx] {
            '0' ..= '9' => { 
                num = num * 10 + (chars[*idx] as i64 - '0' as i64);
                *idx += 1;
            }
            _ => break,
        }
    }

    return Token::Num(num);
}
