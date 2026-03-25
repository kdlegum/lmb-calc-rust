#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Bool(bool),
    Ident(String),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
}
#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Is,
}

pub enum Statement {
    Let(String, Expr),
    Return(Expr),
}

pub type Program = Vec<Statement>;