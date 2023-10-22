extern crate unicode_xid;
use crate::token::Token;
pub mod unescape;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    next_pos: usize,
    ch: char
}


fn is_id_start(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || c == '_'
    || c == '$'
    || c == '¥'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}


fn is_id_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || ('0'..='9').contains(&c)
    || c == '_'
    || c == '$'
    || c == '¥'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}

impl Lexer {

    pub fn new(origin_input: &str) -> Self {
        let input = origin_input.chars().collect::<Vec<char>>();
        let mut lexer = Self {
            input,
            pos: 0,
            next_pos: 0,
            ch: '\0',
        };

        lexer.walk_char();

        lexer
    }

    fn walk_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => {
                if self.next_is('=') {
                    self.walk_char();
                    Token::LTEQ
                } else {
                    Token::LT
                }
            },
            '>' => {
                if self.next_is('=') {
                    self.walk_char();
                    Token::GTEQ
                } else {
                    Token::GT
                }
            },
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => {
                if self.next_is('=') {
                    self.walk_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            '!' => {
                if self.next_is('=') {
                    self.walk_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },
            '\0' => Token::Eof,
            '"' => {
                return self.consume_string();
            },
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '.' => Token::Dot,
            ':' => Token::Colon,
            '0'..='9' => {
                return self.consume_number()
            },
            '\n' => {
                if self.next_is('\n') {
                    Token::Blank
                } else {
                    self.walk_char();
                    return self.next_token();
                }
            }
            _ => {
                if is_id_start(self.ch) {
                    return self.consume_identifier();
                } else {
                    Token::Illegal
                }
            }
        };

        self.walk_char();
        tok
    }

    fn next_is(&mut self, ch: char) -> bool {
        self.next_ch() == ch
    }

    fn next_ch(&mut self) -> char {
        if self.next_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_pos]
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if {
                matches!(
                    self.ch,
                    // Usual ASCII suspects
                    '\u{0009}'   // \t
                    | '\u{000C}' // form feed
                    | '\u{000D}' // \r
                    | '\u{000B}' // vertical tab
                    | '\u{0020}' // space
            
                    // NEXT LINE from latin1
                    | '\u{0085}'
            
                    // Bidi markers
                    | '\u{200E}' // LEFT-TO-RIGHT MARK
                    | '\u{200F}' // RIGHT-TO-LEFT MARK
            
                    // Dedicated whitespace characters from Unicode
                    | '\u{2028}' // LINE SEPARATOR
                    | '\u{2029}' // PARAGRAPH SEPARATOR
                )
            } {
                self.walk_char();
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            if is_id_continue(self.ch) {
                self.walk_char();
            } else {
                break;
            }
        }

        let end_pos = self.pos;

        let literal = self.input[start_pos..end_pos].iter().collect::<String>();

        match literal.as_str() {
            "let" => Token::Let,
            "给" => Token::Let,
            "以" => Token::Assign,
            "前进" => Token::Plus,
            "降维" => Token::Minus,
            "const" => Token::Const,
            "思想钢印" => Token::Const,
            "fn" => Token::Function,
            "法则" => Token::Function,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "这是计划的一部分" => Token::Bool(true),
            "主不在乎" => Token::Bool(false),
            "if" => Token::If,
            "while" => Token::While,
            "else" => Token::Else,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "return" => Token::Return,
            "面壁" => Token::While,
            "破壁" => Token::Break,
            "延绪" => Token::Continue,
            _ => {
                Token::Ident(literal)
            }
        }
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                '0'..='9' => {
                    self.walk_char();
                }
                _ => {
                    break;
                }
            }
        }

        let end_pos = self.pos;

        let literal = &self.input[start_pos..end_pos].iter().collect::<String>();
        Token::Int(literal.parse::<i64>().unwrap())
    }

    fn consume_string(&mut self) -> Token {
        self.walk_char();
        let start_pos = self.pos;

        loop {
            if self.ch == '"' {
                let end_pos = self.pos;
                let literal = self.input[start_pos..end_pos].iter().collect::<String>();
                self.walk_char();
                return Token::String(literal);
            } else {
                self.walk_char();
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::Lexer;
    // use crate::Token;

    #[test]
    fn test_lexer_walk() {
        let mut lexer = Lexer::new(r"let five = 5;");
        assert_eq!(lexer.pos, 0);
        assert_eq!(lexer.next_pos, 1);
        assert_eq!(lexer.ch, 'l');
    }

    #[test]
    fn test_let_five_token() {
        let mut lexer = Lexer::new(r"let five = 5;");
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident("five".to_owned()));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
10 < 10;
10 > 10;
"foobar";
"foo bar";

[1, 2];


{"foo": "bar"};

.
"#;


        let tests = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Blank,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Blank,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::GT,
            Token::Int(5),
            Token::Semicolon,
            Token::Blank,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::RBrace,
            Token::Blank,
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::Int(10),
            Token::LT,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::GT,
            Token::Int(10),
            Token::Semicolon,
            Token::String(String::from("foobar")),
            Token::Semicolon,
            Token::String(String::from("foo bar")),
            Token::Semicolon,
            Token::Blank,
            Token::LBracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::RBracket,
            Token::Semicolon,
            Token::Blank,
            Token::Blank,
            Token::LBrace,
            Token::String(String::from("foo")),
            Token::Colon,
            Token::String(String::from("bar")),
            Token::RBrace,
            Token::Semicolon,
            Token::Blank,
            Token::Dot,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(tok, expect);
        }
    }
}