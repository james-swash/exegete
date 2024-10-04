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

    pub fn lookup_ident(ident: &str) -> TokenType {
        if ident == "fn" {
            FUNCTION
        } else if ident == "let" {
            LET
        } else {
            IDENT
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
            todo!()
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
                current_char = self.ch.expect("Never None");
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
                    '{' => tok = token::Token::new(token::LBRACE, current_char.to_string()),
                    '}' => tok = token::Token::new(token::RBRACE, current_char.to_string()),
                    _ => {
                        if current_char.is_letter() {
                            let ident = self.read_identifier();
                            println!("{ident}");
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
}

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::token;

    #[test]
    fn basic_next_token() {
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

    #[test]
    fn intermediate_next_token() {
        let input_code = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
"#;
        let expected: Vec<(token::TokenType, &str)> = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, ")"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::EOF, ""),
        ];
        let mut lexer = Lexer::new(input_code);
        for (exp_token, exp_literal) in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token.kind, exp_token);
            assert_eq!(&token.literal, exp_literal);
        }
    }
}
