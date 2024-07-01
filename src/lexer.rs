
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Parenthesis(char),
    CurlyBracket(char),
    SquareBracket(char),
    Num(i64),
    Str(String),
    BackSlash,
    DoubleRightArrow,
    SingleRightArrow,
    SingleLeftArrow,
    Comma,
    SemiColon,
    Colon,
    UnaryOperator(String),
    BinaryOperator(String),
    Keyword(String),
}

pub fn lex(code_str: &str) -> Vec<Token> {

    let mut idx: usize = 0;
    let mut result_tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = code_str.chars().collect();

    while idx < chars.len() {
        match chars[idx] {

            'a' ..= 'z' | 'A'..='Z' | '_' => {
                result_tokens.push(parse_keyword_or_identifier(&chars, &mut idx));
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

            '"' => {
                result_tokens.push(parse_string(&chars, &mut idx));
            }

            '\\' => {
                result_tokens.push(Token::BackSlash);
                idx += 1;
            }

            '-' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '>' {
                    result_tokens.push(Token::SingleRightArrow);
                    idx += 2;
                } else {
                    result_tokens.push(Token::BinaryOperator("-".to_string()));
                    idx += 1;
                }
            }

            '+' => {
                result_tokens.push(Token::BinaryOperator("+".to_string()));
                idx += 1;
            }

            '*' => {
                result_tokens.push(Token::BinaryOperator("*".to_string()));
                idx += 1;
            }

            '/' => {
                result_tokens.push(Token::BinaryOperator("/".to_string()));
                idx += 1;
            }

            '!' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    result_tokens.push(Token::BinaryOperator("!=".to_string()));
                    idx += 2;
                } else {
                    result_tokens.push(Token::UnaryOperator("!".to_string()));
                    idx += 1;
                }
            }

            '=' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '>' {
                    result_tokens.push(Token::DoubleRightArrow);
                    idx += 2;
                } else if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    result_tokens.push(Token::BinaryOperator("==".to_string()));
                    idx += 2;
                } else {
                    result_tokens.push(Token::BinaryOperator("=".to_string()));
                    idx += 1;
                }
            }

            '<' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '-' {
                    result_tokens.push(Token::SingleLeftArrow);
                    idx += 2;
                } else if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    result_tokens.push(Token::BinaryOperator("<=".to_string()));
                    idx += 2;
                } else {
                    result_tokens.push(Token::BinaryOperator("<".to_string()));
                    idx += 1;
                }
            }

            '>' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    result_tokens.push(Token::BinaryOperator(">=".to_string()));
                    idx += 2;
                } else {
                    result_tokens.push(Token::BinaryOperator(">".to_string()));
                    idx += 1;
                }
            }

            ',' => {
                result_tokens.push(Token::Comma);
                idx += 1;
            }

            ';' => {
                result_tokens.push(Token::SemiColon);
                idx += 1;
            }

            ':' => {
                result_tokens.push(Token::Colon);
                idx += 1;
            }

            _ => {
                idx += 1;
            }

        }

    }

    return result_tokens;
}

fn parse_string(chars: &[char], idx: &mut usize) -> Token {
    let mut string = String::new();
    *idx += 1;
    while *idx < chars.len() {
        match chars[*idx] {
            '"' => {
                *idx += 1;
                break;
            }
            _ => {
                string.push(chars[*idx]);
                *idx += 1;
            }
        }
    }

    return Token::Str(string);
}


fn parse_keyword_or_identifier(chars: &[char], idx: &mut usize) -> Token {

    const KEYWORDS: [&str; 6] = ["if", "then", "else", "struct", "variant", "return"];
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

    if KEYWORDS.contains(&identifier.as_str()) {
        return Token::Keyword(identifier);
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
