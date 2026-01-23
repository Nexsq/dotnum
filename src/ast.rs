#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    Str(String),
    Bool(bool),
    Var(String),
    Unary(Op, Box<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Or,
    Not,
}

#[derive(Clone, Debug)]
pub enum Node {
    VarDecl { name: String, value: Expr },
    Assign { name: String, value: Expr },
    Call { name: String, args: Vec<Expr> },
    Loop { times: Expr, body: Vec<Node> },
    While { cond: Expr, body: Vec<Node> },
    If { cond: Expr, then_body: Vec<Node>, else_body: Option<Vec<Node>> },
    Break,
    Continue,
}