mod token {
    pub const ILLEGAL: TokenType = TokenType("ILLEGAL");
    pub const EOF: TokenType = TokenType("EOF");
    pub const IDENT: TokenType = TokenType("IDENT");
    pub const INT: TokenType = TokenType("INT");
    pub const ASSIGN: TokenType = TokenType("=");
    pub const PLUS: TokenType = TokenType("+");
    pub const COMMA: TokenType = TokenType(",");
    pub const SEMICOLON: TokenType = TokenType(";");
    pub const LPAREN: TokenType = TokenType("(");
    pub const RPAREN: TokenType = TokenType(")");
    pub const LBRACE: TokenType = TokenType("{");
    pub const RBRACE: TokenType = TokenType("}");
    pub const FUNCTION: TokenType = TokenType("FUNCTION");
    pub const LET: TokenType = TokenType("LET");

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TokenType(&'static str);

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Token {
        pub kind: TokenType,
        pub literal: String,
    }

    impl Token {
        pub fn new(kind: TokenType, literal: String) -> Self {
            Token { kind, literal }
        }
    }
}

mod lexer {
    use crate::token;
    #[derive(Debug)]
    pub struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: Option<char>,
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
        pub fn next_token(&mut self) -> token::Token {
            let tok: token::Token;
            if let Some(current_char) = self.ch {
                match current_char {
                    '=' => tok = token::Token::new(token::ASSIGN, current_char.to_string()),
                    ';' => tok = token::Token::new(token::SEMICOLON, current_char.to_string()),
                    '(' => tok = token::Token::new(token::LPAREN, current_char.to_string()),
                    ')' => tok = token::Token::new(token::RPAREN, current_char.to_string()),
                    ',' => tok = token::Token::new(token::COMMA, current_char.to_string()),
                    '+' => tok = token::Token::new(token::PLUS, current_char.to_string()),
                    '{' => tok = token::Token::new(token::LBRACE, current_char.to_string()),
                    '}' => tok = token::Token::new(token::RBRACE, current_char.to_string()),
                    _ => panic!("We don't handle this token yet!"),
                }
            } else {
                tok = token::Token::new(token::EOF, "".to_string());
            }
            self.read_char();
            tok
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::token;

    #[test]
    fn test_next_token() {
        let input_code = "=+(){},;";
        let expected: Vec<(token::TokenType, &str)> = vec![
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
        ];
        let mut lexer = Lexer::new(input_code);
        for (exp_token, exp_literal) in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token.kind, exp_token);
            assert_eq!(&token.literal, exp_literal);
        }
    }
}
