#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    Str(String),
    Var(String),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Op {
    Eq, Ne, Gt, Lt, Ge, Le, And, Or,
}

#[derive(Clone, Debug)]
pub enum Node {
    VarDecl { name: String, value: Expr },
    Assign { name: String, value: Expr },
    Call { name: String, args: Vec<Expr> },
    Loop { times: Expr, body: Vec<Node> },
    If { cond: Expr, then_body: Vec<Node>, else_body: Option<Vec<Node>> },
}