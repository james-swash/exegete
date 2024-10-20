#[cfg(test)]
use exegete::lexer::Lexer;
use exegete::token;

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
fn numbers_and_idents_test() {
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
        (token::RBRACE, "}"),
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

#[test]
fn more_identifiers_test() {
    let input_code = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
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
        (token::RBRACE, "}"),
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
        (token::BANG, "!"),
        (token::MINUS, "-"),
        (token::SLASH, "/"),
        (token::ASTERISK, "*"),
        (token::INT, "5"),
        (token::SEMICOLON, ";"),
        (token::INT, "5"),
        (token::LT, "<"),
        (token::INT, "10"),
        (token::GT, ">"),
        (token::INT, "5"),
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
