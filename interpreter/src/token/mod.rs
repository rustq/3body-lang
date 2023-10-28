#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // identifiers + literals
    Ident(String),
    Int(i64),

    // operations
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    // condition
    While,

    LT,
    GT,
    LTEQ,
    GTEQ,

    // Delimiters
    Comma,
    Semicolon,

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Dot,

    // Keywords
    Function,
    Let,
    Const,
    True,
    False,
    If,
    Else,
    Break,
    Continue,
    Return,

    Equal,
    NotEqual,

    String(String),
    Bool(bool),

    LBracket,
    RBracket,

    Colon,

    Blank,
}


#[cfg(test)]
mod tests {
    use super::Token;
    #[test]
    fn test_token_able_to_eq() {
        assert_eq!(Token::Let, Token::Let)
    }
}