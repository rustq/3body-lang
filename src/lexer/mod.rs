extern crate unicode_normalization;
extern crate unicode_xid;
use crate::token::Token;

pub mod unescape;

/// All variable names are nfc-normaized.
pub fn nfc_normalize(string: &str) -> String {
    use self::unicode_normalization::{is_nfc_quick, IsNormalized, UnicodeNormalization};
    match is_nfc_quick(string.chars()) {
        IsNormalized::Yes => String::from(string),
        _ => {
            let normalized_str: String = string.chars().nfc().collect();
            String::from(&normalized_str)
        }
    }
}

pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space minus \n.

    matches!(
        c,
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
}

/// Decide whether character may show up in emoji.
/// We cannot validate the entire sequence given the current architecture.
fn is_emoji_like(c: char) -> bool {
    if c < '\x7f' {
        false
    } else {
        // ZWJ
        c == '\u{200D}'
        // VS15, 16
        || c == '\u{fe0f}' || c == '\u{fe0e}'
        // Big SMP chunk (includes modifiers and by accident chess)
        || ('\u{1f000}'..='\u{1faff}').contains(&c)
        // The BMP parts that follow are actually quite questionable
        || c == '\u{2139}'
        // (unstable!) Arrows, not sure if we will repocess them for operators!
        || ('\u{2190}'..='\u{21FF}').contains(&c)
        || ('\u{2300}'..='\u{23FF}').contains(&c)
        || ('\u{25A0}'..='\u{25FF}').contains(&c)
        || ('\u{2600}'..='\u{26FF}').contains(&c)
        // (unstable!) Dingbats, some are unfortunately punctuations
        || ('\u{2700}'..='\u{27FF}').contains(&c)
        // Too lazy to do 2800-329f, will come back later
    }
}

/// True if `c` is valid as a first character of an identifier.
/// Compared to Rust, we additionally allow $ and ¥.
fn is_id_start(c: char) -> bool {
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || c == '_'
        || c == '$'
        || c == '¥'
        || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
        || is_emoji_like(c)
}

/// True if `c` is valid as a non-first character of an identifier.
/// Compared to Rust, we additionally allow $ and ¥.
fn is_id_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || ('0'..='9').contains(&c)
        || c == '_'
        || c == '$'
        || c == '¥'
        || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_continue(c))
        || is_emoji_like(c)
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(origin_input: &str) -> Self {
        let input = origin_input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: '\0',
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn nextch(&mut self) -> char {
        if self.next_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_pos]
        }
    }

    fn nextch_is(&mut self, ch: char) -> bool {
        self.nextch() == ch
    }

    fn skip_whitespace(&mut self) {
        loop {
            if is_whitespace(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.nextch_is('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.nextch_is('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => {
                if self.nextch_is('=') {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.nextch_is('=') {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '.' => Token::Dot,
            '[' => Token::Lbracket,
            ']' => Token::Rbracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '0'..='9' => {
                return self.consume_number();
            }
            '"' => {
                return self.consume_string();
            }
            '\n' => {
                if self.nextch_is('\n') {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            '\0' => Token::Eof,
            _ => {
                if is_id_start(self.ch) {
                    return self.consume_identifier();
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();

        tok
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            if is_id_continue(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        let literal = self.input[start_pos..self.pos].iter().collect::<String>();

        match literal.as_str() {
            "fn" => Token::Func,
            "let" => Token::Let,
            "给" => Token::Let,
            "以" => Token::Assign,
            "前进" => Token::Plus,
            "降维" => Token::Minus,
            "法则" => Token::Func,
            "这是计划的一部分" => Token::Bool(true),
            "主不在乎" => Token::Bool(false),
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "while" => Token::While,
            "else" => Token::Else,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "面壁" => Token::While,
            "破壁" => Token::Break,
            "延绪" => Token::Continue,
            _ => Token::Ident(nfc_normalize(&literal)),
        }
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                '0'..='9' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos].iter().collect::<String>();

        Token::Int(literal.parse::<i64>().unwrap())
    }

    fn consume_string(&mut self) -> Token {
        self.read_char();

        let start_pos = self.pos;
        let mut bs = false;

        while self.ch != '\0' {
            if bs {
                bs = false;
            } else {
                match self.ch {
                    '"' => {
                        let literal = self.input[start_pos..self.pos].iter().collect::<String>();
                        self.read_char();
                        return Token::String(unescape::unescape_str_or_byte_str_all(&literal));
                    }
                    '\\' => {
                        bs = true;
                    }
                    _ => (),
                }
            }
            self.read_char();
        }
        // FIXME: Make Lexer faliable
        Token::String("<Lexer error: string: premature EOF>".to_string())
    }
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use token::Token;

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
10 <= 10;
10 >= 10;
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
            Token::Func,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Blank,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::Blank,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::Rbrace,
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
            Token::LessThanEqual,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::GreaterThanEqual,
            Token::Int(10),
            Token::Semicolon,
            Token::String(String::from("foobar")),
            Token::Semicolon,
            Token::String(String::from("foo bar")),
            Token::Semicolon,
            Token::Blank,
            Token::Lbracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::Rbracket,
            Token::Semicolon,
            Token::Blank,
            Token::Blank,
            Token::Lbrace,
            Token::String(String::from("foo")),
            Token::Colon,
            Token::String(String::from("bar")),
            Token::Rbrace,
            Token::Semicolon,
            Token::Blank,
            Token::Dot,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}
