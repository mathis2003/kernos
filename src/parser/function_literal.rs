use crate::parser::parser_utils::{ParseResult, eat_whitespace};
use crate::parser::expression::{parse_expression, Expression};
use crate::parser::identifier::{parse_identifier, Identifier};
//use crate::parser::number::{parse_number, Number};
//use crate::parser::keyword::{parse_keyword, Keyword};



#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    ReturnStatement(Expression),
    AssignmentStatement(String, Expression),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionLiteral {
    parameters: Vec<String>,
    body: Vec<Statement>,
}

// Take a slice of chars and return an AST node and the number of chars consumed
pub fn parse_function_literal(chars: &[char]) -> ParseResult<FunctionLiteral> {
    let mut idx: usize = 0;
    let mut function_literal = FunctionLiteral {
        parameters: Vec::new(),
        body: Vec::new(),
    };

    idx += eat_whitespace(&chars[idx..]);

    // parse
    if chars.get(idx) != Some(&'\\') {
        return ParseResult::Failure("Expected lambda symbol (i.e.: backslash) at the beginning of function literal".to_string(), 0);
    }

    idx += 1; // skip over the backslash

    // parse the parameters
    loop {
        idx += eat_whitespace(&chars[idx..]);
        if &chars[idx..idx+2] == "->".to_string().chars().collect::<Vec<char>>().as_slice() {
            break;
        }
        match parse_identifier(&chars[idx..]) {
            ParseResult::Success(Identifier(s), offset) => {
                function_literal.parameters.push(s.clone());
                idx += offset;
            }
            ParseResult::Failure(msg, depth) => {
                return ParseResult::Failure(msg, depth+1);
            }
        }

        idx += eat_whitespace(&chars[idx..]);
        if chars.get(idx) == Some(&',') {
            idx += 1;
        }
    }

    // we are now on the single right arrow token, skip over it
    idx += 2;
    idx += eat_whitespace(&chars[idx..]); // whitespace: \   ->

    if chars.get(idx) != Some(&'{') {
        return ParseResult::Failure("Expected curly bracket after right arrow".to_string(), 0);
    }
    idx += 1; // skip over the opening curly bracket token

    // parse the body
    loop {
        idx += eat_whitespace(&chars[idx..]);
        if chars.get(idx) == Some(&'}') {
            break;
        }
        match parse_statement(&chars[idx..]) {
            ParseResult::Failure(msg, depth) => {
                return ParseResult::Failure(msg, depth+1);
            }
            ParseResult::Success(statement, new_idx) => {
                idx += new_idx;
                function_literal.body.push(statement);
            }
        }
    }

    // we are now on the closing curly bracket token, skip over it
    idx += 1;
    idx += eat_whitespace(&chars[idx..]);

    ParseResult::Success(function_literal, idx)
}

pub fn parse_statement(chars: &[char]) -> ParseResult<Statement> {
    let mut idx: usize = 0;
    let statement: Statement;

    idx += eat_whitespace(&chars[idx..]);

    if &chars[idx..idx+6] == "return".to_string().chars().collect::<Vec<char>>().as_slice() {
        idx += 6;
        idx += eat_whitespace(&chars[idx..]); // whitespace: return    expression
        match parse_expression(&chars[idx..]) {
            ParseResult::Failure(msg, depth) => {
                return ParseResult::Failure(msg, depth+1);
            }
            ParseResult::Success(expression, new_idx) => {
                idx += new_idx;
                statement = Statement::ReturnStatement(expression);
            }
        }
    } else if let ParseResult::Success(Identifier(id), offset) = parse_identifier(&chars[idx..]) {
        idx += offset;
        if &chars[idx..idx+2] == "<-".to_string().chars().collect::<Vec<char>>().as_slice() {
            idx += 2;
            idx += eat_whitespace(&chars[idx..]); // whitespace: id <- expression
            match parse_expression(&chars[idx..]) {
                ParseResult::Failure(msg, depth) => {
                    return ParseResult::Failure(msg, depth+1);
                }
                ParseResult::Success(expression, new_idx) => {
                    idx += new_idx;
                    statement = Statement::AssignmentStatement(id, expression);
                }
            }
        } else {
            return ParseResult::Failure("Expected left arrow after identifier".to_string(), 0);
        }
    } else {
        return ParseResult::Failure("Expected return statement or identifier at the beginning of statement".to_string(), 0);
    }

    if chars.get(idx) != Some(&';') {
        return ParseResult::Failure("Expected semicolon at the end of statement".to_string(), 0);
    }

    idx += 1; // skip over the semicolon

    idx += eat_whitespace(&chars[idx..]);

    ParseResult::Success(statement, idx)
}