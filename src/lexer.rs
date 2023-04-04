use crate::token::Token;
use crate::token::KEYWORDS;

use Token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            '=' => tok = ASSIGN,
            ';' => tok = SEMICOLON,
            '(' => tok = LPAREN,
            ')' => tok = RPAREN,
            ',' => tok = COMMA,
            '+' => tok = PLUS,
            '{' => tok = LBRACE,
            '}' => tok = RBRACE,
            '\0' => tok = EOF,
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_identifier();
                if KEYWORDS.contains_key(ident.as_str()) {
                    return KEYWORDS[ident.as_str()].clone();
                }
                return IDENT(ident)
            },
            '0'..='9' => {
                let int = self.read_number();
                return INT(int);
            },
            _ => tok = ILLEGAL,
        }
        self.read_char();
        tok
    }

    fn read_number(&mut self) -> i64 {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].parse().expect("not a number!")
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
}

//test mod
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_character_tokens() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF,
        ];
        for expected_token in expected {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#;
        let mut lexer = Lexer::new(input);
        let expected = vec![
            LET,
            IDENT("five".to_string()),
            ASSIGN,
            INT(5),
            SEMICOLON,
            LET,
            IDENT("ten".to_string()),
            ASSIGN,
            INT(10),
            SEMICOLON,
            LET,
            IDENT("add".to_string()),
            ASSIGN,
            FUNCTION,
            LPAREN,
            IDENT("x".to_string()),
            COMMA,
            IDENT("y".to_string()),
            RPAREN,
            LBRACE,
            IDENT("x".to_string()),
            PLUS,
            IDENT("y".to_string()),
            SEMICOLON,
            RBRACE,
            SEMICOLON,
            LET,
            IDENT("result".to_string()),
            ASSIGN,
            IDENT("add".to_string()),
            LPAREN,
            IDENT("five".to_string()),
            COMMA,
            IDENT("ten".to_string()),
            RPAREN,
            SEMICOLON,
            EOF,
        ];
        for expected_token in expected {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }
}
