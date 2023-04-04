use crate::token::Token;
use crate::token::KEYWORDS;

use Token::*;

#[derive(Debug)]
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
            '-' => tok = MINUS,
            '!' => tok = BANG,
            '*' => tok = ASTERISK,
            '/' => tok = SLASH,
            '<' => tok = LT,
            '>' => tok = GT,
            '{' => tok = LBRACE,
            '}' => tok = RBRACE,
            '\0' => tok = EOF,
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_identifier();
                if KEYWORDS.contains_key(ident.as_str()) {
                    return KEYWORDS[ident.as_str()].clone();
                }
                return IDENT(ident);
            }
            '0'..='9' => {
                let int = self.read_number();
                return INT(int);
            }
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
        self.input[position..self.position]
            .parse()
            .expect("not a number!")
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

    fn print_current_position(&self) {
        let mut line = 0;
        let mut line_start = 0;
        let mut line_end = 0;
        let mut has_position = false;
        let mut column = 0;
        let mut last_column =0;
        for (i, c) in self.input.chars().enumerate() {
            if i == self.position {
                has_position = true;
                last_column = column;
            }
            if c == '\n' {
                if has_position {
                    line_end = i;
                    break;
                }
                line_start = i + 1;
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        println!("lexer: {:#?}", self);
        println!("input: ({}:{})", line, column);
        unsafe {
            println!("{}", self.input.slice_unchecked(line_start, line_end));
        }
        println!("{}^", " ".repeat(last_column));
        println!("{}|", " ".repeat(last_column));
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
        !-/*5;
        5 < 10 > 5;
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
            BANG,
            MINUS,
            SLASH,
            ASTERISK,
            INT(5),
            SEMICOLON,
            INT(5),
            LT,
            INT(10),
            GT,
            INT(5),
            SEMICOLON,
            EOF,
        ];
        for expected_token in expected {
            let token = lexer.next_token();
            assert_eq!(token, expected_token, "lexer state: {:#?}", lexer.print_current_position());
        }
    }
}
