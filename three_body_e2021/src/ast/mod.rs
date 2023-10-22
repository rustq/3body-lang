
#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
    String(String),
    Bool(bool),
    Array(Vec<Expr>),
    Hash(Vec<(Expr, Expr)>),
}

/// prefix
#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Not => write!(f, "!"),
        }
    }
}

/// infix
#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GTEQ,
    GT,
    LTEQ,
    LT,
}

impl std::fmt::Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "=="),
            Infix::NotEqual => write!(f, "!="),
            Infix::GTEQ => write!(f, ">="),
            Infix::GT => write!(f, ">"),
            Infix::LTEQ => write!(f, "<="),
            Infix::LT => write!(f, "<"),
        }
    }
}

/// expr
#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    While {
        cond: Box<Expr>,
        consequence: BlockStmt,
    },
    If {
        cond: Box<Expr>,
        consequence: BlockStmt,
        alternative: Option<BlockStmt>,
    },
    Function {
        params: Vec<Ident>,
        body: BlockStmt,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Let(Ident, Expr),
    Const(Ident, Expr),
    Break,
    Blank,
    Continue,
    Return(Expr),
    Expr(Expr),
    ReAssign(Ident, Expr)
}

pub type BlockStmt = Vec<Stmt>;

pub type Program = BlockStmt;


///
// Precedence low -> high
// 优先级从低到高
///
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
    Index,       // array[index]
}
