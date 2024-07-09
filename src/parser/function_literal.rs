use crate::parser::parser_utils::ParseResult;
use crate::parser::expression::{parse_expression, Expression};
use crate::lexer::Token;



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

// Take a slice of tokens and return an AST node and the number of tokens consumed
pub fn parse_function_literal(tokens: &[Token]) -> ParseResult<FunctionLiteral> {
    let mut idx: usize = 0;
    let mut function_literal = FunctionLiteral {
        parameters: Vec::new(),
        body: Vec::new(),
    };

    // parse
    if tokens.get(idx) != Some(&Token::BackSlash) {
        return ParseResult::Failure("Expected lambda symbol (i.e.: backslash) at the beginning of function literal".to_string(), 0);
    }

    // parse the parameters
    idx += 1;
    loop {
        match &tokens[idx] {
            Token::Identifier(s) => {
                function_literal.parameters.push(s.clone());
                idx += 1;
                match tokens.get(idx) {
                    Some(Token::Comma) => {
                        idx += 1;
                    }
                    Some(Token::SingleRightArrow) => {
                        break;
                    }
                    _ => return ParseResult::Failure("Expected comma or right arrow after identifier".to_string(), 0),
                }
            }
            Token::SingleRightArrow => {
                break;
            }
            _ => return ParseResult::Failure("Expected identifier at the beginning of parameter list".to_string(), 0),
        }
    }

    // we are now on the single right arrow token, skip over it
    idx += 1;
    if tokens.get(idx) != Some(&Token::CurlyBracket('{')) {
        return ParseResult::Failure("Expected curly bracket after right arrow".to_string(), 0);
    }
    idx += 1;

    // parse the body
    loop {
        if tokens.get(idx) == Some(&Token::CurlyBracket('}')) {
            break;
        }
        match parse_statement(&tokens[idx..]) {
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

    ParseResult::Success(function_literal, idx)
}

pub fn parse_statement(tokens: &[Token]) -> ParseResult<Statement> {
    let mut idx: usize = 0;
    let statement: Statement;

    match &tokens[idx] {
        Token::Keyword(s) => {
            if s == "return" {
                idx += 1;
                match parse_expression(&tokens[idx..]) {
                    ParseResult::Failure(msg, depth) => {
                        return ParseResult::Failure(msg, depth+1);
                    }
                    ParseResult::Success(expression, new_idx) => {
                        idx += new_idx;
                        statement = Statement::ReturnStatement(expression);
                    }
                }
            } else {
                return ParseResult::Failure("Expected return statement at the beginning of statement".to_string(), 0);
            }
        }
        Token::Identifier(s) => {
            let id = s.clone();
            idx += 1;
            if tokens.get(idx) == Some(&Token::SingleLeftArrow) {
                idx += 1;
                match parse_expression(&tokens[idx..]) {
                    ParseResult::Failure(msg, depth) => {
                        return ParseResult::Failure(msg, depth+1);
                    }
                    ParseResult::Success(expression, new_idx) => {
                        idx += new_idx;
                        statement = Statement::AssignmentStatement(id, expression);
                    }
                }
            } else {
                return ParseResult::Failure("Expected assignment operator after identifier".to_string(), 0);
            }
        }
        _ => return ParseResult::Failure("Expected return statement or identifier at the beginning of statement".to_string(), 0),
    }

    if tokens.get(idx) != Some(&Token::SemiColon) {
        return ParseResult::Failure("Expected semicolon at the end of statement".to_string(), 0);
    }

    idx += 1;

    ParseResult::Success(statement, idx)
}