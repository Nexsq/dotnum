#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Var, If, Else, Loop,
    Ident(String),
    Number(i64),
    Str(String),
    LParen, RParen, LBrace, RBrace,
    Comma, Semicolon,
    Eq, EqEq, Ne, Gt, Lt, Ge, Le,
    AndAnd, OrOr,
    Eof,
}