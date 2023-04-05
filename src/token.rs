use phf::{phf_map, Map};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    EQ,
    NotEQ,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    //Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
use Token::*;

impl Token {
    pub fn literal(&self) -> String {
        match self {
            ILLEGAL => "ILLEGAL".to_string(),
            EOF => "EOF".to_string(),
            IDENT(s) => s.to_string(),
            INT(i) => i.to_string(),
            ASSIGN => "=".to_string(),
            EQ => "==".to_string(),
            NotEQ => "!=".to_string(),
            PLUS => "+".to_string(),
            MINUS => "-".to_string(),
            BANG => "!".to_string(),
            ASTERISK => "*".to_string(),
            SLASH => "/".to_string(),
            LT => "<".to_string(),
            GT => ">".to_string(),
            COMMA => ",".to_string(),
            SEMICOLON => ";".to_string(),
            LPAREN => "(".to_string(),
            RPAREN => ")".to_string(),
            LBRACE => "{".to_string(),
            RBRACE => "}".to_string(),
            FUNCTION => "fn".to_string(),
            LET => "let".to_string(),
            TRUE => "true".to_string(),
            FALSE => "false".to_string(),
            IF => "if".to_string(),
            ELSE => "else".to_string(),
            RETURN => "return".to_string(),
        }
    }
}

pub static KEYWORDS: Map<&'static str, Token> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "if" => IF,
    "else" => ELSE,
    "return" => RETURN,
};
