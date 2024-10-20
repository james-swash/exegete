pub const ILLEGAL: TokenType = TokenType("ILLEGAL");
pub const EOF: TokenType = TokenType("EOF");
pub const IDENT: TokenType = TokenType("IDENT");
pub const INT: TokenType = TokenType("INT");
pub const ASSIGN: TokenType = TokenType("=");
pub const PLUS: TokenType = TokenType("+");
pub const MINUS: TokenType = TokenType("-");
pub const BANG: TokenType = TokenType("!");
pub const ASTERISK: TokenType = TokenType("*");
pub const SLASH: TokenType = TokenType("/");
pub const LT: TokenType = TokenType("<");
pub const GT: TokenType = TokenType(">");
pub const COMMA: TokenType = TokenType(",");
pub const SEMICOLON: TokenType = TokenType(";");
pub const LPAREN: TokenType = TokenType("(");
pub const RPAREN: TokenType = TokenType(")");
pub const LBRACE: TokenType = TokenType("{");
pub const RBRACE: TokenType = TokenType("}");
pub const FUNCTION: TokenType = TokenType("FUNCTION");
pub const LET: TokenType = TokenType("LET");
pub const TRUE: TokenType = TokenType("TRUE");
pub const FALSE: TokenType = TokenType("FALSE");
pub const IF: TokenType = TokenType("IF");
pub const ELSE: TokenType = TokenType("ELSE");
pub const RETURN: TokenType = TokenType("RETURN");

#[derive(Debug, PartialEq)]
pub struct TokenType(&'static str);

#[derive(Debug, PartialEq)]
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
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        _ => IDENT,
    }
}
