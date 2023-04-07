use crate::token;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: Vec::new() }
    }

    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}


impl Statement {
    fn token_literal(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}



impl LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}
