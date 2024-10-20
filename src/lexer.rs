use crate::token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        'a' <= *self && *self <= 'z' || 'A' <= *self && *self <= 'Z' || *self == '_'
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn read_number(&mut self) -> String {
        let position = self.position;
        while self
            .ch
            .expect("self.ch should always be Some variant at this point")
            .is_ascii_digit()
        {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self
            .ch
            .expect("self.ch should always be a Some variant at this point")
            .is_letter()
        {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    fn skip_whitespace(&mut self) {
        let mut current_char = self.ch.expect("Never None here");
        while current_char == ' '
            || current_char == '\t'
            || current_char == '\n'
            || current_char == '\r'
        {
            self.read_char();
            if let Some(some_char) = self.ch {
                current_char = some_char
            } else {
                break;
            }
        }
    }
    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
        if let Some(current_char) = self.ch {
            match current_char {
                '=' => tok = token::Token::new(token::ASSIGN, current_char.to_string()),
                ';' => tok = token::Token::new(token::SEMICOLON, current_char.to_string()),
                '(' => tok = token::Token::new(token::LPAREN, current_char.to_string()),
                ')' => tok = token::Token::new(token::RPAREN, current_char.to_string()),
                ',' => tok = token::Token::new(token::COMMA, current_char.to_string()),
                '+' => tok = token::Token::new(token::PLUS, current_char.to_string()),
                '-' => tok = token::Token::new(token::MINUS, current_char.to_string()),
                '!' => tok = token::Token::new(token::BANG, current_char.to_string()),
                '*' => tok = token::Token::new(token::ASTERISK, current_char.to_string()),
                '/' => tok = token::Token::new(token::SLASH, current_char.to_string()),
                '<' => tok = token::Token::new(token::LT, current_char.to_string()),
                '>' => tok = token::Token::new(token::GT, current_char.to_string()),
                '{' => tok = token::Token::new(token::LBRACE, current_char.to_string()),
                '}' => tok = token::Token::new(token::RBRACE, current_char.to_string()),
                _ => {
                    if current_char.is_letter() {
                        let ident = self.read_identifier();
                        let ident_kind = token::lookup_ident(&ident);
                        return token::Token::new(ident_kind, ident);
                    } else if current_char.is_ascii_digit() {
                        let number = self.read_number();
                        return token::Token::new(token::INT, number);
                    } else {
                        tok = token::Token::new(token::ILLEGAL, current_char.to_string())
                    }
                }
            }
        } else {
            tok = token::Token::new(token::EOF, "".to_string());
        }
        self.read_char();
        tok
    }
}
