use crate::ast::{Expression, Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();

        while self.cur_token != Token::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            _ => Err(format!("no parse function for {:?}", self.cur_token)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();
        match self.peek_token {
            Token::IDENT(_) => self.next_token(),
            _ => {
                return Err(format!(
                    "expected next token to be IDENT, got {:?}",
                    self.peek_token
                ))
            }
        };

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal(),
        };

        match self.peek_token {
            Token::ASSIGN => self.next_token(),
            _ => {
                return Err(format!(
                    "expected next token to be ASSIGN, got {:?}",
                    self.cur_token
                ))
            }
        }

        let expression = Expression::Identifier(Identifier {
            token: self.peek_token.clone(),
            value: self.peek_token.literal(),
        });

        // skipping the expression until we encounter a semicolon
        while self.cur_token != Token::SEMICOLON {
            self.next_token();
        }

        Ok(Statement::LetStatement(LetStatement {
            token,
            name,
            value: expression,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Identifier, LetStatement, Statement};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected_program = Program {
            statements: vec![
                Statement::LetStatement(LetStatement {
                    token: Token::LET,
                    name: Identifier {
                        token: Token::IDENT("x".to_string()),
                        value: "x".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::INT(5),
                        value: "5".to_string(),
                    }),
                }),
                Statement::LetStatement(LetStatement {
                    token: Token::LET,
                    name: Identifier {
                        token: Token::IDENT("y".to_string()),
                        value: "y".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::INT(10),
                        value: "10".to_string(),
                    }),
                }),
                Statement::LetStatement(LetStatement {
                    token: Token::LET,
                    name: Identifier {
                        token: Token::IDENT("foobar".to_string()),
                        value: "foobar".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::INT(838383),
                        value: "838383".to_string(),
                    }),
                }),
            ],
        };

        let program = parser.parse_program();
        let program = program.expect("parse_program() returned an error");

        assert_eq!(program, expected_program);
    }
}
