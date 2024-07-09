#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseResult<T> {
    Success(T, usize), // expression, index in source code tokens array
    Failure(String, usize), // error message, depth of the parse error
}