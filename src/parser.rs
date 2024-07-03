use crate::lexer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i64),
    FunctionLiteral(FunctionLiteral),
}

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
pub fn parse_function_literal(tokens: &[Token]) -> (FunctionLiteral, usize) {
    let mut idx: usize = 0;
    let mut function_literal = FunctionLiteral {
        parameters: Vec::new(),
        body: Vec::new(),
    };

    // parse
    if tokens.get(idx) != Some(&Token::BackSlash) {
        panic!("Expected lambda symbol (i.e.: backslash) at the beginning of function literal");
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
                    _ => panic!("Expected comma or right arrow after identifier"),
                }
            }
            Token::SingleRightArrow => {
                break;
            }
            _ => panic!("Expected identifier at the beginning of parameter list"),
        }
    }

    // we are now on the single right arrow token, skip over it
    idx += 1;
    if tokens.get(idx) != Some(&Token::CurlyBracket('{')) {
        panic!("Expected curly bracket after right arrow");
    }
    idx += 1;

    // parse the body
    loop {
        if tokens.get(idx) == Some(&Token::CurlyBracket('}')) {
            break;
        }
        let (statement, new_idx) = parse_statement(&tokens[idx..]);
        idx += new_idx;
        function_literal.body.push(statement);
    }

    // we are now on the closing curly bracket token, skip over it
    idx += 1;

    (function_literal, idx)
}

pub fn parse_statement(tokens: &[Token]) -> (Statement, usize) {
    let mut idx: usize = 0;
    let statement: Statement;

    match &tokens[idx] {
        Token::Keyword(s) => {
            if s == "return" {
                idx += 1;
                let (expression, new_idx) = parse_expression(&tokens[idx..]);
                idx += new_idx;
                statement = Statement::ReturnStatement(expression);
            } else {
                panic!("Expected return statement at the beginning of statement");
            }
        }
        Token::Identifier(s) => {
            let id = s.clone();
            idx += 1;
            if tokens.get(idx) == Some(&Token::SingleLeftArrow) {
                idx += 1;
                let (expression, new_idx) = parse_expression(&tokens[idx..]);
                idx += new_idx;
                statement = Statement::AssignmentStatement(id, expression);
            } else {
                panic!("Expected assignment operator after identifier");
            }
        }
        _ => panic!("Expected return statement or identifier at the beginning of statement"),
    }

    if tokens.get(idx) != Some(&Token::SemiColon) {
        panic!("Expected semicolon at the end of statement");
    }

    idx += 1;

    (statement, idx)
}

pub fn parse_expression(tokens: &[Token]) -> (Expression, usize) {
    let mut idx: usize = 0;
    let expression: Expression;

    match &tokens[idx] {
        Token::Identifier(s) => {
            expression = Expression::Identifier(s.clone());
            idx += 1;
        }
        Token::Num(n) => {
            expression = Expression::NumberLiteral(*n);
            idx += 1;
        }
        Token::BackSlash => {
            let (function_literal, new_idx) = parse_function_literal(&tokens[idx..]);
            idx += new_idx;
            expression = Expression::FunctionLiteral(function_literal);
        }
        _ => panic!("Expected identifier, number literal, or function literal at the beginning of expression"),
    }

    (expression, idx)
}
