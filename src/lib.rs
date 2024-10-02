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
}

mod lexer {
    use crate::token;

    pub struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: char,
    }

    impl Lexer {
        pub fn new(input: &str) -> Self {
            let mut lexer = Self {
                input: input.to_string(),
                position: 0,
                read_position: 0,
                ch: ' ',
            };
            lexer.read_char();
            lexer
        }
        fn read_char(&mut self) {
            if self.read_position >= self.input.len() {
                self.ch = ' ';
            } else {
                self.ch = self.input.chars().nth(self.read_position).unwrap();
            }
            self.position = self.read_position;
            self.read_position += 1;
        }
        pub fn next_token(&self) -> token::Token {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::token;
    use std::collections::HashMap;

    #[test]
    fn test_next_token() {
        let input_code = "=+(){},;";
        let expected = HashMap::from([
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
        ]);
        let lexer = Lexer::new(input_code);
        for (exp_token, exp_literal) in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token.kind, exp_token);
            assert_eq!(&token.literal, exp_literal);
        }
    }
}
