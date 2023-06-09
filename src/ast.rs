use crate::token;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }

    pub fn string(&self) -> String {
        let mut out = "".to_string();
        let mut out = Vec::new();
        for statement in &self.statements {
            //out.push_str(&statement.string());
            out.push(statement.string());
        }
        out.join("\n")
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Statement {
    fn token_literal(&self) -> String {
        "".to_string()
    }
    fn string(&self) -> String {
        let mut out = match self {
            Statement::LetStatement(let_statement) => let_statement.string(),
            Statement::ReturnStatement(return_statement) => return_statement.string(),
            Statement::ExpressionStatement(expression_statement) => expression_statement.string(),
        };
        out
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
    fn string(&self) -> String {
        let mut out = "".to_string();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        out.push_str(&self.value.string());
        out.push_str(";");
        out
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn string(&self) -> String {
        let mut out = "".to_string();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.return_value.string());
        out.push_str(";");
        out
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}

impl Expression {
    fn token_literal(&self) -> String {
        "".to_string()
    }
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn string(&self) -> String {
        let mut out = "".to_string();
        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        out.push_str(&self.right.string());
        out.push_str(")");
        out
    }
}

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn string(&self) -> String {
        let mut out = "".to_string();
        out.push_str("(");
        out.push_str(&self.operator);
        out.push_str(&self.right.string());
        out.push_str(")");
        out
    }
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    pub fn string(&self) -> String {
        self.token.literal()
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn string(&self) -> String {
        match &self.expression {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_string() {
        let program_string = r#"
let x =     5;
let y    = 10;

let foobar = another_identifier;
"#;

        let expected = r#"let x = 5;
let y = 10;
let foobar = another_identifier;"#;

        let lexer = Lexer::new(program_string);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.string(), expected);
    }
}
