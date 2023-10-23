use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
    errors: ParseErrors,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken { want: Option<Token>, got: Token },
}

pub type ParseErrors = Vec<ParseError>;

///
// Basic Implement
///
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
            errors: vec![],
        };

        parser.walk_token();
        parser.walk_token();

        parser
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while self.current_token != Token::Eof {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.walk_token();
        }

        program
    }

    /// The entry of parse stmt
    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Let => self.parse_let_stmt(),
            Token::Const => self.parse_const_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::Ident(_) => match self.next_token {
                Token::Assign => self.parse_reassign_stmt(),
                _ => self.parse_expr_stmt(),
            },
            Token::Break => self.parse_break_stmt(),
            Token::Continue => self.parse_continue_stmt(),
            Token::Blank => Some(Stmt::Blank),
            _ => self.parse_expr_stmt(),
        }
    }
}

///
// Walk Token and Assert Implement
///
impl Parser {
    fn walk_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }

    fn next_token_is(&mut self, tok: Token) -> bool {
        self.next_token == tok
    }

    fn next_token_borrow_is(&mut self, token_borrow: &Token) -> bool {
        self.next_token == *token_borrow
    }

    fn assert_next_token(&mut self, tok: Token) -> bool {
        if self.next_token == tok {
            true
        } else {
            self.error_next_token(tok);
            false
        }
    }

    fn error_next_token(&mut self, tok: Token) {
        self.errors.push(ParseError::UnexpectedToken {
            want: Some(tok),
            got: self.next_token.clone(),
        });
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.errors.clone()
    }

    fn error_no_prefix_parser(&mut self) {
        self.errors.push(ParseError::UnexpectedToken {
            want: None,
            got: self.next_token.clone(),
        });
    }
}

///
// Stmt Assign Implement
///
impl Parser {

    /// let
    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::Ident(_) => self.walk_token(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.assert_next_token(Token::Assign) {
            return None;
        }

        self.walk_token();
        self.walk_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Let(name, expr))
    }

    /// const
    fn parse_const_stmt(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::Ident(_) => self.walk_token(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.assert_next_token(Token::Assign) {
            return None;
        }

        self.walk_token();
        self.walk_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Const(name, expr))
    }

    /// reassign
    fn parse_reassign_stmt(&mut self) -> Option<Stmt> {
        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.assert_next_token(Token::Assign) {
            return None;
        }

        self.walk_token();
        self.walk_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::ReAssign(name, expr))
    }
}

///
// Condition Parsing Implement
///
impl Parser {
    /// if expr
    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.assert_next_token(Token::LParen) {
            return None;
        }

        self.walk_token();
        self.walk_token();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.assert_next_token(Token::RParen) {
            return None;
        }
        self.walk_token();

        if !self.assert_next_token(Token::LBrace) {
            return None;
        }
        self.walk_token();

        let consequence = self.parse_block_stmt();

        let mut alternative = None;

        if self.next_token_is(Token::Else) {
            self.walk_token();
            if !self.assert_next_token(Token::LBrace) {
                return None;
            }
            self.walk_token();

            alternative = Some(self.parse_block_stmt());
        }

        Some(Expr::If {
            cond: Box::new(cond),
            consequence,
            alternative,
        })
    }

    /// while expr
    fn parse_while_expr(&mut self) -> Option<Expr> {
        if !self.assert_next_token(Token::LParen) {
            return None;
        }

        self.walk_token();
        self.walk_token();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.assert_next_token(Token::RParen) {
            return None;
        }
        self.walk_token();

        if !self.assert_next_token(Token::LBrace) {
            return None;
        }
        self.walk_token();

        let consequence = self.parse_block_stmt();

        Some(Expr::While {
            cond: Box::new(cond),
            consequence,
        })
    }

    /// break
    fn parse_break_stmt(&mut self) -> Option<Stmt> {
        self.walk_token();

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Break)
    }

    /// continue
    fn parse_continue_stmt(&mut self) -> Option<Stmt> {
        self.walk_token();

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Continue)
    }

    /// return
    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.walk_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            _ => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Return(expr))
    }

    fn parse_block_stmt(&mut self) -> BlockStmt {
        self.walk_token();

        let mut block: Vec<Stmt> = vec![];

        while !self.current_token_is(Token::RBrace) {
            if self.current_token_is(Token::Eof) {
                self.error_next_token(Token::RBrace);
                return block;
            }
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                None => {}
            }
            self.walk_token();
        }

        block
    }
}

///
// Expr Parsing Implement
///
impl Parser {
    /// expr
    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            _ => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.walk_token();
        }

        Some(Stmt::Expr(expr))
    }

    /// parse expr ...
    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = match self.current_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            Token::String(_) => self.parse_string_expr(),
            Token::Bool(_) => self.parse_bool_expr(),
            Token::LBracket => self.parse_array_expr(),
            Token::LBrace => self.parse_hash_expr(),
            Token::LParen => self.parse_grouped_expr(),
            Token::Bang | Token::Minus | Token::Plus => self.parse_prefix_expr(),
            Token::If => self.parse_if_expr(),
            Token::While => self.parse_while_expr(),
            Token::Function => self.parse_function_expr(),
            _ => {
                self.error_no_prefix_parser();
                None
            }
        };

        //
        // recursive to parse the higher precedence right expr
        // which means "不断递归地 向 更高优先级的 右表达式 结合"
        // 例如 a + b / c 这个用例 会不断地向更右递归结合 -> 直到返回
        //
        while !self.next_token_is(Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LT
                | Token::LTEQ
                | Token::GT
                | Token::GTEQ => {
                    self.walk_token();
                    left = self.parse_infix_expr(left.unwrap());
                }
                Token::LBracket => {
                    self.walk_token();
                    left = self.parse_index_expr(left.unwrap());
                }
                Token::Dot => {
                    self.walk_token();
                    left = self.parse_dot_index_expr(left.unwrap());
                }
                Token::LParen => {
                    self.walk_token();
                    left = self.parse_call_expr(left.unwrap());
                }
                _ => return left,
            }
        }
        // // todo

        left
    }

    /// ident expr
    fn parse_ident_expr(&mut self) -> Option<Expr> {
        self.parse_ident().map(Expr::Ident)
    }

    /// int expr
    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Int(ref mut int) => Some(Expr::Literal(Literal::Int(*int))),
            _ => None,
        }
    }

    /// string expr
    fn parse_string_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::String(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => None,
        }
    }

    /// boolean expr
    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Bool(value) => Some(Expr::Literal(Literal::Bool(value))),
            _ => None,
        }
    }

    /// array expr
    fn parse_array_expr(&mut self) -> Option<Expr> {
        self.parse_expr_list(Token::RBracket)
            .map(|list| Expr::Literal(Literal::Array(list)))
    }

    /// hash expr
    fn parse_hash_expr(&mut self) -> Option<Expr> {
        let mut pairs = Vec::new();

        while !self.next_token_is(Token::RBrace) {
            self.walk_token();

            let key = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            if !self.assert_next_token(Token::Colon) {
                return None;
            }

            self.walk_token();
            self.walk_token();

            let value = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            pairs.push((key, value));

            if !self.next_token_is(Token::RBrace) {
                if !self.assert_next_token(Token::Comma) {
                    return None;
                }
                self.walk_token();
            }
        }

        if !self.assert_next_token(Token::RBrace) {
            self.walk_token();
            return None;
        }

        self.walk_token();

        Some(Expr::Literal(Literal::Hash(pairs)))
    }

    /// parse the expr list until get the end token
    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
        let mut list = vec![];

        if self.next_token_borrow_is(&end) {
            self.walk_token();
            return Some(list);
        }

        self.walk_token();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }

        while self.next_token_is(Token::Comma) {
            self.walk_token();
            self.walk_token();
            if self.current_token_is(Token::Eof) {
                self.error_next_token(Token::Ident(String::from("ident")));
                return None;
            }

            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }

        if !self.assert_next_token(end) {
            return None;
        }

        self.walk_token();

        Some(list)
    }

    /// prefix expr
    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.current_token {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.walk_token();

        self.parse_expr(Precedence::Prefix)
            .map(|expr| Expr::Prefix(prefix, Box::new(expr)))
    }

    /// infix expr (which means "中缀-表达式")
    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.current_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Equal => Infix::Equal,
            Token::NotEqual => Infix::NotEqual,
            Token::LT => Infix::LT,
            Token::LTEQ => Infix::LTEQ,
            Token::GT => Infix::GT,
            Token::GTEQ => Infix::GTEQ,
            _ => return None,
        };

        let precedence = self.current_token_precedence();

        self.walk_token();

        self.parse_expr(precedence)
            .map(|right_expr| Expr::Infix(infix, Box::new(left), Box::new(right_expr)))
    }

    /// index expr
    fn parse_index_expr(&mut self, left: Expr) -> Option<Expr> {
        self.walk_token();

        let index = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.assert_next_token(Token::RBracket) {
            return None;
        }

        self.walk_token();

        Some(Expr::Index(Box::new(left), Box::new(index)))
    }

    /// dot index expr
    fn parse_dot_index_expr(&mut self, left: Expr) -> Option<Expr> {
        self.walk_token();

        match self.parse_ident() {
            Some(name) => match name {
                Ident(str) => Some(Expr::Index(
                    Box::new(left),
                    Box::new(Expr::Literal(Literal::String(str))),
                )),
                _ => return None,
            },
            None => return None,
        }
    }

    /// group expr
    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.walk_token();

        let expr = self.parse_expr(Precedence::Lowest);

        if !self.assert_next_token(Token::RParen) {
            None
        } else {
            self.walk_token();
            expr
        }
    }
}

///
// Function Parsing Implement
///
impl Parser {
    /// function expr
    fn parse_function_expr(&mut self) -> Option<Expr> {
        if !self.assert_next_token(Token::LParen) {
            return None;
        }

        self.walk_token();

        let params = match self.parse_function_args() {
            Some(params) => params,
            None => return None,
        };

        if !self.assert_next_token(Token::LBrace) {
            return None;
        }

        self.walk_token();

        Some(Expr::Function {
            params,
            body: self.parse_block_stmt(),
        })
    }

    /// function args
    fn parse_function_args(&mut self) -> Option<Vec<Ident>> {
        let mut args = vec![];

        if self.next_token_is(Token::RParen) {
            self.walk_token();
            return Some(args);
        }

        self.walk_token();

        match self.parse_ident() {
            Some(ident) => args.push(ident),
            None => return None,
        };

        while self.next_token_is(Token::Comma) {
            self.walk_token();
            self.walk_token();

            match self.parse_ident() {
                Some(ident) => args.push(ident),
                None => return None,
            };
        }

        if !self.assert_next_token(Token::RParen) {
            return None;
        }

        self.walk_token();

        Some(args)
    }

    fn parse_call_expr(&mut self, func_name: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::RParen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func_name),
            args,
        })
    }
}

///
// Precedence Parsing Implement (which means "运算优先级")
///
impl Parser {
    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LT | Token::LTEQ => Precedence::LessGreater,
            Token::GT | Token::GTEQ => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LBracket => Precedence::Index,
            Token::Dot => Precedence::Index,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }
}

///
// ...
///
impl Parser {
    /// all block stmt

    ///
    /// ident
    ///
    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;
    use crate::ast::Ident;
    use crate::ast::Infix;
    use crate::ast::Literal;
    use crate::ast::Prefix;
    use crate::ast::Stmt;

    use super::Lexer;
    use super::Parser;

    ///
    // cases from 2015
    // cases from 2015
    ///

    fn check_parse_errors(parser: &mut Parser) {
        let errors = parser.get_errors();

        if errors.is_empty() {
            return;
        }

        println!("\n");

        println!("parser has {} errors", errors.len());

        for err in errors {
            println!("parse error: {:?}", err);
        }

        println!("\n");

        panic!("failed");
    }

    #[test]
    fn test_blank() {
        let input = r#"
1000;

1000;


1000;

if (x) {

    x;

}
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Blank,
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Blank,
                Stmt::Blank,
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Blank,
                Stmt::Expr(Expr::If {
                    cond: Box::new(Expr::Ident(Ident(String::from("x")))),
                    consequence: vec![
                        Stmt::Blank,
                        Stmt::Expr(Expr::Ident(Ident(String::from("x")))),
                        Stmt::Blank,
                    ],
                    alternative: None,
                }),
            ],
            program,
        );
    }

    #[test]
    fn test_let_stmt() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![
                Stmt::Let(Ident(String::from("x")), Expr::Literal(Literal::Int(5))),
                Stmt::Let(Ident(String::from("y")), Expr::Literal(Literal::Int(10))),
                Stmt::Let(
                    Ident(String::from("foobar")),
                    Expr::Literal(Literal::Int(838383)),
                ),
            ],
            program,
        );
    }

    #[test]
    fn test_const_stmt() {
        let input = r#"
const x = 5;
const y = 10;
const foobar = 838383;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![
                Stmt::Const(Ident(String::from("x")), Expr::Literal(Literal::Int(5))),
                Stmt::Const(Ident(String::from("y")), Expr::Literal(Literal::Int(10))),
                Stmt::Const(
                    Ident(String::from("foobar")),
                    Expr::Literal(Literal::Int(838383)),
                ),
            ],
            program,
        );
    }

    #[test]
    fn test_reassign_stmt() {
        let input = r#"
x = 5;
y = 10;
foobar = 838383;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![
                Stmt::ReAssign(Ident(String::from("x")), Expr::Literal(Literal::Int(5))),
                Stmt::ReAssign(Ident(String::from("y")), Expr::Literal(Literal::Int(10))),
                Stmt::ReAssign(
                    Ident(String::from("foobar")),
                    Expr::Literal(Literal::Int(838383)),
                ),
            ],
            program,
        );
    }

    #[test]
    fn test_return_stmt() {
        let input = r#"
return 5;
return 10;
return 993322;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![
                Stmt::Return(Expr::Literal(Literal::Int(5))),
                Stmt::Return(Expr::Literal(Literal::Int(10))),
                Stmt::Return(Expr::Literal(Literal::Int(993322))),
            ],
            program,
        );
    }

    #[test]
    fn test_ident_expr() {
        let input = "foobar;";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Ident(Ident(String::from("foobar"))))],
            program,
        );
    }

    #[test]
    fn test_integer_literal_expr() {
        let input = "5;";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(vec![Stmt::Expr(Expr::Literal(Literal::Int(5)))], program,);
    }

    #[test]
    fn test_string_literal_expr() {
        let input = "\"hello world\";";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Literal(Literal::String(String::from(
                "hello world",
            ))))],
            program,
        );
    }

    #[test]
    fn test_boolean_literal_expr() {
        let tests = vec![
            ("true;", Stmt::Expr(Expr::Literal(Literal::Bool(true)))),
            ("false;", Stmt::Expr(Expr::Literal(Literal::Bool(false)))),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }

    #[test]
    fn test_array_literal_expr() {
        let input = "[1, 2 * 2, 3 + 3]";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Literal(Literal::Array(vec![
                Expr::Literal(Literal::Int(1)),
                Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Literal(Literal::Int(2))),
                    Box::new(Expr::Literal(Literal::Int(2))),
                ),
                Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(3))),
                    Box::new(Expr::Literal(Literal::Int(3))),
                ),
            ])))],
            program,
        );
    }

    #[test]
    fn test_hash_literal_expr() {
        let tests = vec![
            ("{}", Stmt::Expr(Expr::Literal(Literal::Hash(vec![])))),
            (
                "{\"one\": 1, \"two\": 2, \"three\": 3}",
                Stmt::Expr(Expr::Literal(Literal::Hash(vec![
                    (
                        Expr::Literal(Literal::String(String::from("one"))),
                        Expr::Literal(Literal::Int(1)),
                    ),
                    (
                        Expr::Literal(Literal::String(String::from("two"))),
                        Expr::Literal(Literal::Int(2)),
                    ),
                    (
                        Expr::Literal(Literal::String(String::from("three"))),
                        Expr::Literal(Literal::Int(3)),
                    ),
                ]))),
            ),
            (
                "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}",
                Stmt::Expr(Expr::Literal(Literal::Hash(vec![
                    (
                        Expr::Literal(Literal::String(String::from("one"))),
                        Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Literal(Literal::Int(0))),
                            Box::new(Expr::Literal(Literal::Int(1))),
                        ),
                    ),
                    (
                        Expr::Literal(Literal::String(String::from("two"))),
                        Expr::Infix(
                            Infix::Minus,
                            Box::new(Expr::Literal(Literal::Int(10))),
                            Box::new(Expr::Literal(Literal::Int(8))),
                        ),
                    ),
                    (
                        Expr::Literal(Literal::String(String::from("three"))),
                        Expr::Infix(
                            Infix::Divide,
                            Box::new(Expr::Literal(Literal::Int(15))),
                            Box::new(Expr::Literal(Literal::Int(5))),
                        ),
                    ),
                ]))),
            ),
            (
                "{key: \"value\"}",
                Stmt::Expr(Expr::Literal(Literal::Hash(vec![(
                    Expr::Ident(Ident(String::from("key"))),
                    Expr::Literal(Literal::String(String::from("value"))),
                )]))),
            ),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }

    #[test]
    fn test_index_expr() {
        let input = "myArray[1 + 1]";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Index(
                Box::new(Expr::Ident(Ident(String::from("myArray")))),
                Box::new(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ))],
            program
        );
    }

    #[test]
    fn test_dot_index_expr() {
        let input = "myHash.key";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Index(
                Box::new(Expr::Ident(Ident(String::from("myHash")))),
                Box::new(Expr::Literal(Literal::String(String::from("key")))),
            ))],
            program
        );
    }

    #[test]
    fn test_prefix_expr() {
        let tests = vec![
            (
                "!5;",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Not,
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "-15;",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Minus,
                    Box::new(Expr::Literal(Literal::Int(15))),
                )),
            ),
            (
                "+15;",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Plus,
                    Box::new(Expr::Literal(Literal::Int(15))),
                )),
            ),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }

    #[test]
    fn test_infix_expr() {
        let tests = vec![
            (
                "5 + 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 - 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 * 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 / 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 > 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::GT,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 < 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::LT,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 == 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 != 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::NotEqual,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 >= 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::GTEQ,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
            (
                "5 <= 5;",
                Stmt::Expr(Expr::Infix(
                    Infix::LTEQ,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                )),
            ),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }

    #[test]
    fn test_if_expr() {
        let input = "if (x < y) { x }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::If {
                cond: Box::new(Expr::Infix(
                    Infix::LT,
                    Box::new(Expr::Ident(Ident(String::from("x")))),
                    Box::new(Expr::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Stmt::Expr(Expr::Ident(Ident(String::from("x"))))],
                alternative: None,
            })],
            program,
        );
    }

    #[test]
    fn test_if_else_expr() {
        let input = "if (x < y) { x } else { y }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::If {
                cond: Box::new(Expr::Infix(
                    Infix::LT,
                    Box::new(Expr::Ident(Ident(String::from("x")))),
                    Box::new(Expr::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Stmt::Expr(Expr::Ident(Ident(String::from("x"))))],
                alternative: Some(vec![Stmt::Expr(Expr::Ident(Ident(String::from("y"))))]),
            })],
            program,
        );
    }

    #[test]
    fn test_function_expr() {
        let input = "fn(x, y) { x + y; }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Function {
                params: vec![Ident(String::from("x")), Ident(String::from("y"))],
                body: vec![Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Ident(Ident(String::from("x")))),
                    Box::new(Expr::Ident(Ident(String::from("y")))),
                ))],
            })],
            program,
        );
    }

    #[test]
    fn test_function_args() {
        let tests = vec![
            ("fn() {};", vec![]),
            ("fn(x) {};", vec![Ident(String::from("x"))]),
            (
                "fn(x, y, z) {};",
                vec![
                    Ident(String::from("x")),
                    Ident(String::from("y")),
                    Ident(String::from("z")),
                ],
            ),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(
                vec![Stmt::Expr(Expr::Function {
                    params: expect,
                    body: vec![],
                })],
                program,
            );
        }
    }

    #[test]
    fn test_call_expr() {
        let input = "add(1, 2 * 3, 4 + 5);";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::Call {
                func: Box::new(Expr::Ident(Ident(String::from("add")))),
                args: vec![
                    Expr::Literal(Literal::Int(1)),
                    Expr::Infix(
                        Infix::Multiply,
                        Box::new(Expr::Literal(Literal::Int(2))),
                        Box::new(Expr::Literal(Literal::Int(3))),
                    ),
                    Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(4))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    ),
                ],
            })],
            program,
        );
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            (
                "-a * b",
                Stmt::Expr(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Prefix(
                        Prefix::Minus,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("b")))),
                )),
            ),
            (
                "!-a",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Not,
                    Box::new(Expr::Prefix(
                        Prefix::Minus,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                    )),
                )),
            ),
            (
                "a + b + c",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("c")))),
                )),
            ),
            (
                "a + b - c",
                Stmt::Expr(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("c")))),
                )),
            ),
            (
                "a * b * c",
                Stmt::Expr(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Infix(
                        Infix::Multiply,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("c")))),
                )),
            ),
            (
                "a * b / c",
                Stmt::Expr(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Infix(
                        Infix::Multiply,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("c")))),
                )),
            ),
            (
                "a + b / c",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Ident(Ident(String::from("a")))),
                    Box::new(Expr::Infix(
                        Infix::Divide,
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                        Box::new(Expr::Ident(Ident(String::from("c")))),
                    )),
                )),
            ),
            (
                "a + b * c + d / e - f",
                Stmt::Expr(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Ident(Ident(String::from("a")))),
                            Box::new(Expr::Infix(
                                Infix::Multiply,
                                Box::new(Expr::Ident(Ident(String::from("b")))),
                                Box::new(Expr::Ident(Ident(String::from("c")))),
                            )),
                        )),
                        Box::new(Expr::Infix(
                            Infix::Divide,
                            Box::new(Expr::Ident(Ident(String::from("d")))),
                            Box::new(Expr::Ident(Ident(String::from("e")))),
                        )),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("f")))),
                )),
            ),
            (
                "5 > 4 == 3 < 4",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Infix(
                        Infix::GT,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                    Box::new(Expr::Infix(
                        Infix::LT,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
            ),
            (
                "5 < 4 != 3 > 4",
                Stmt::Expr(Expr::Infix(
                    Infix::NotEqual,
                    Box::new(Expr::Infix(
                        Infix::LT,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                    Box::new(Expr::Infix(
                        Infix::GT,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
            ),
            (
                "5 >= 4 == 3 <= 4",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Infix(
                        Infix::GTEQ,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                    Box::new(Expr::Infix(
                        Infix::LTEQ,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
            ),
            (
                "5 <= 4 != 3 >= 4",
                Stmt::Expr(Expr::Infix(
                    Infix::NotEqual,
                    Box::new(Expr::Infix(
                        Infix::LTEQ,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                    Box::new(Expr::Infix(
                        Infix::GTEQ,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(4))),
                    )),
                )),
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Literal(Literal::Int(4))),
                            Box::new(Expr::Literal(Literal::Int(5))),
                        )),
                    )),
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Literal(Literal::Int(3))),
                            Box::new(Expr::Literal(Literal::Int(1))),
                        )),
                        Box::new(Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Literal(Literal::Int(4))),
                            Box::new(Expr::Literal(Literal::Int(5))),
                        )),
                    )),
                )),
            ),
            ("true", Stmt::Expr(Expr::Literal(Literal::Bool(true)))),
            ("false", Stmt::Expr(Expr::Literal(Literal::Bool(false)))),
            (
                "3 > 5 == false",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Infix(
                        Infix::GT,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    )),
                    Box::new(Expr::Literal(Literal::Bool(false))),
                )),
            ),
            (
                "3 < 5 == true",
                Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Infix(
                        Infix::LT,
                        Box::new(Expr::Literal(Literal::Int(3))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    )),
                    Box::new(Expr::Literal(Literal::Bool(true))),
                )),
            ),
            (
                "1 + (2 + 3) + 4",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(1))),
                        Box::new(Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Literal(Literal::Int(2))),
                            Box::new(Expr::Literal(Literal::Int(3))),
                        )),
                    )),
                    Box::new(Expr::Literal(Literal::Int(4))),
                )),
            ),
            (
                "(5 + 5) * 2",
                Stmt::Expr(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    )),
                    Box::new(Expr::Literal(Literal::Int(2))),
                )),
            ),
            (
                "2 / (5 + 5)",
                Stmt::Expr(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Literal(Literal::Int(2))),
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    )),
                )),
            ),
            (
                "-(5 + 5)",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Minus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Literal(Literal::Int(5))),
                        Box::new(Expr::Literal(Literal::Int(5))),
                    )),
                )),
            ),
            (
                "!(true == true)",
                Stmt::Expr(Expr::Prefix(
                    Prefix::Not,
                    Box::new(Expr::Infix(
                        Infix::Equal,
                        Box::new(Expr::Literal(Literal::Bool(true))),
                        Box::new(Expr::Literal(Literal::Bool(true))),
                    )),
                )),
            ),
            (
                "d + add(b * c)",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Ident(Ident(String::from("d")))),
                    Box::new(Expr::Call {
                        func: Box::new(Expr::Ident(Ident(String::from("add")))),
                        args: vec![Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Ident(Ident(String::from("b")))),
                            Box::new(Expr::Ident(Ident(String::from("c")))),
                        )],
                    }),
                )),
            ),
            (
                "a + add(b * c) + d",
                Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Call {
                            func: Box::new(Expr::Ident(Ident(String::from("add")))),
                            args: vec![Expr::Infix(
                                Infix::Multiply,
                                Box::new(Expr::Ident(Ident(String::from("b")))),
                                Box::new(Expr::Ident(Ident(String::from("c")))),
                            )],
                        }),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("d")))),
                )),
            ),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident(Ident(String::from("add")))),
                    args: vec![
                        Expr::Ident(Ident(String::from("a"))),
                        Expr::Ident(Ident(String::from("b"))),
                        Expr::Literal(Literal::Int(1)),
                        Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Literal(Literal::Int(2))),
                            Box::new(Expr::Literal(Literal::Int(3))),
                        ),
                        Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Literal(Literal::Int(4))),
                            Box::new(Expr::Literal(Literal::Int(5))),
                        ),
                        Expr::Call {
                            func: Box::new(Expr::Ident(Ident(String::from("add")))),
                            args: vec![
                                Expr::Literal(Literal::Int(6)),
                                Expr::Infix(
                                    Infix::Multiply,
                                    Box::new(Expr::Literal(Literal::Int(7))),
                                    Box::new(Expr::Literal(Literal::Int(8))),
                                ),
                            ],
                        },
                    ],
                }),
            ),
            (
                "add(a + b + c * d / f + g)",
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident(Ident(String::from("add")))),
                    args: vec![Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Infix(
                                Infix::Plus,
                                Box::new(Expr::Ident(Ident(String::from("a")))),
                                Box::new(Expr::Ident(Ident(String::from("b")))),
                            )),
                            Box::new(Expr::Infix(
                                Infix::Divide,
                                Box::new(Expr::Infix(
                                    Infix::Multiply,
                                    Box::new(Expr::Ident(Ident(String::from("c")))),
                                    Box::new(Expr::Ident(Ident(String::from("d")))),
                                )),
                                Box::new(Expr::Ident(Ident(String::from("f")))),
                            )),
                        )),
                        Box::new(Expr::Ident(Ident(String::from("g")))),
                    )],
                }),
            ),
            (
                "a * [1, 2, 3, 4][b * c] * d",
                Stmt::Expr(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Infix(
                        Infix::Multiply,
                        Box::new(Expr::Ident(Ident(String::from("a")))),
                        Box::new(Expr::Index(
                            Box::new(Expr::Literal(Literal::Array(vec![
                                Expr::Literal(Literal::Int(1)),
                                Expr::Literal(Literal::Int(2)),
                                Expr::Literal(Literal::Int(3)),
                                Expr::Literal(Literal::Int(4)),
                            ]))),
                            Box::new(Expr::Infix(
                                Infix::Multiply,
                                Box::new(Expr::Ident(Ident(String::from("b")))),
                                Box::new(Expr::Ident(Ident(String::from("c")))),
                            )),
                        )),
                    )),
                    Box::new(Expr::Ident(Ident(String::from("d")))),
                )),
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident(Ident(String::from("add")))),
                    args: vec![
                        Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Ident(Ident(String::from("a")))),
                            Box::new(Expr::Index(
                                Box::new(Expr::Ident(Ident(String::from("b")))),
                                Box::new(Expr::Literal(Literal::Int(2))),
                            )),
                        ),
                        Expr::Index(
                            Box::new(Expr::Ident(Ident(String::from("b")))),
                            Box::new(Expr::Literal(Literal::Int(1))),
                        ),
                        Expr::Infix(
                            Infix::Multiply,
                            Box::new(Expr::Literal(Literal::Int(2))),
                            Box::new(Expr::Index(
                                Box::new(Expr::Literal(Literal::Array(vec![
                                    Expr::Literal(Literal::Int(1)),
                                    Expr::Literal(Literal::Int(2)),
                                ]))),
                                Box::new(Expr::Literal(Literal::Int(1))),
                            )),
                        ),
                    ],
                }),
            ),
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }

    // self cases

    #[test]
    fn test_while_expr() {
        let input = "while (x < y) { b = b + 10; }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::While {
                cond: Box::new(Expr::Infix(
                    Infix::LT,
                    Box::new(Expr::Ident(Ident(String::from("x")))),
                    Box::new(Expr::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Stmt::ReAssign(
                    Ident(String::from("b")),
                    Expr::Infix(
                        Infix::Plus,
                        Box::new(Expr::Ident(Ident(String::from("b")))),
                        Box::new(Expr::Literal(Literal::Int(10)))
                    )
                ),],
            })],
            program,
        );
    }

    #[test]
    fn test_while_break_continue_expr() {
        let input = "while (x < y) { b = b + 10; if (b == 30) { break; } else { continue; } }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
        assert_eq!(
            vec![Stmt::Expr(Expr::While {
                cond: Box::new(Expr::Infix(
                    Infix::LT,
                    Box::new(Expr::Ident(Ident(String::from("x")))),
                    Box::new(Expr::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![
                    Stmt::ReAssign(
                        Ident(String::from("b")),
                        Expr::Infix(
                            Infix::Plus,
                            Box::new(Expr::Ident(Ident(String::from("b")))),
                            Box::new(Expr::Literal(Literal::Int(10)))
                        )
                    ),
                    Stmt::Expr(Expr::If {
                        cond: Box::new(Expr::Infix(
                            Infix::Equal,
                            Box::new(Expr::Ident(Ident(String::from("b")))),
                            Box::new(Expr::Literal(Literal::Int(30)))
                        )),
                        consequence: vec![Stmt::Break],
                        alternative: Some(vec![Stmt::Continue]),
                    }),
                ],
            })],
            program,
        );
    }

    /// errors panic

    #[test]
    #[should_panic]
    fn test_let_panic() {
        let input = "let a = 3; let b ! 4;";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_if_panic() {
        let input = "if a { q }";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_else_panic() {
        let input = "if (a) { q } else";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_while_panic() {
        let input = "while (a) { c = 1";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_array_panic() {
        let input = "let list = [3,4,5,";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_hash_panic() {
        let input = "let hash = { \"c\": 3, \"d\" };";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }

    #[test]
    #[should_panic]
    fn test_function_panic() {
        let input = "add(3, 4, 5";

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();

        check_parse_errors(&mut parser);
    }
}
