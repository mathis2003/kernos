#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseResult<T> {
    Success(T, usize), // expression, index in source code tokens array
    Failure(String, usize), // error message, depth of the parse error
}

pub fn eat_whitespace(chars: &[char]) -> usize {
    let mut idx: usize = 0;
    while idx < chars.len() {
        match chars[idx] {
            ' ' | '\n' | '\t' => { idx += 1; }
            _ => { break; }
        }
    }
    idx
}