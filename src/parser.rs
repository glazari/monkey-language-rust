use crate::ast::{
    Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, PrefixExpression,
    Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();

        while self.cur_token != Token::EOF {
            let statement = match self.parse_statement() {
                Ok(statement) => statement,
                Err(e) => {
                    self.errors.push(e);
                    self.skip_statement();
                    self.next_token();
                    continue;
                }
            };
            program.statements.push(statement);
            self.next_token();
        }

        if self.errors.len() != 0 {
            return Err(self.errors.join("\n"));
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn skip_statement(&mut self) {
        while self.cur_token != Token::SEMICOLON {
            self.next_token();
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token == Token::SEMICOLON {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(ExpressionStatement {
            token,
            expression,
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let left_exp = match self.cur_token {
            Token::IDENT(_) => self.parser_identifier(),
            Token::INT(_) => self.parse_integer_literal(),
            Token::BANG | Token::MINUS => self.parse_prefix_expression(),
            _ => Err(format!("no prefix parse function for {:?}", self.cur_token)),
        };
        left_exp
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal();
        self.next_token();

        let right = self.parse_expression(Precedence::PREFIX)?;

        Ok(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, String> {
        let token = self.cur_token.clone();
        let value = match &token {
            Token::INT(value) => value.clone(),
            _ => panic!("expected token to be INT"),
        };

        Ok(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }

    fn parser_identifier(&mut self) -> Result<Expression, String> {
        Ok(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal(),
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone();
        self.next_token();

        let expression = Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal(),
        });

        // skipping the expression until we encounter a semicolon
        self.skip_statement();

        Ok(Statement::ReturnStatement(ReturnStatement {
            token,
            return_value: expression,
        }))
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
                    self.peek_token
                ))
            }
        }

        let expression = Expression::Identifier(Identifier {
            token: self.peek_token.clone(),
            value: self.peek_token.literal(),
        });

        // skipping the expression until we encounter a semicolon
        self.skip_statement();

        Ok(Statement::LetStatement(LetStatement {
            token,
            name,
            value: expression,
        }))
    }
}

enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
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

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(e) => panic!("parse_program() returned an error: {}", e),
        };

        assert_eq!(program, expected_program);
    }

    #[test]
    fn test_let_statement_errors() {
        let input = r#"
let x  5;
let = 10;
let 838383;
"#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_err());
        assert_eq!(
            program.err().unwrap(),
            "expected next token to be ASSIGN, got INT(5)\nexpected next token to be IDENT, got ASSIGN\nexpected next token to be IDENT, got INT(838383)")
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected_program = Program {
            statements: vec![
                Statement::ReturnStatement(ReturnStatement {
                    token: Token::RETURN,
                    return_value: Expression::Identifier(Identifier {
                        token: Token::INT(5),
                        value: "5".to_string(),
                    }),
                }),
                Statement::ReturnStatement(ReturnStatement {
                    token: Token::RETURN,
                    return_value: Expression::Identifier(Identifier {
                        token: Token::INT(10),
                        value: "10".to_string(),
                    }),
                }),
                Statement::ReturnStatement(ReturnStatement {
                    token: Token::RETURN,
                    return_value: Expression::Identifier(Identifier {
                        token: Token::INT(993322),
                        value: "993322".to_string(),
                    }),
                }),
            ],
        };

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(e) => panic!("parse_program() returned an error: {}", e),
        };

        assert_eq!(program, expected_program);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected_program = Program {
            statements: vec![Statement::ExpressionStatement(ExpressionStatement {
                token: Token::IDENT("foobar".to_string()),
                expression: Expression::Identifier(Identifier {
                    token: Token::IDENT("foobar".to_string()),
                    value: "foobar".to_string(),
                }),
            })],
        };

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(e) => panic!("parse_program() returned an error: {}", e),
        };
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected_program = Program {
            statements: vec![Statement::ExpressionStatement(ExpressionStatement {
                token: Token::INT(5),
                expression: Expression::IntegerLiteral(IntegerLiteral {
                    token: Token::INT(5),
                    value: 5,
                }),
            })],
        };

        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(e) => panic!("parse_program() returned an error: {}", e),
        };

        assert_eq!(program, expected_program);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests = vec![
            ("!5;", "!", 5),
            ("-15;", "-", 15),
            //   ("!true;", "!", true),
            //   ("!false;", "!", false),
        ];

        for (input, operator, value) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(e) => panic!("parse_program() returned an error: {}", e),
            };

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];
            let expression = match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    &expression_statement.expression
                }
                _ => panic!("statement is not an expression statement"),
            };

            let prefix_expression = match expression {
                Expression::PrefixExpression(prefix_expression) => prefix_expression,
                _ => panic!("expression is not a prefix expression"),
            };

            assert_eq!(prefix_expression.operator, operator);
            assert_eq!(prefix_expression.right.string(), value.to_string());
        }
    }

}
