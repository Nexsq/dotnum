#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    Str(String),
    Bool(bool),
    Var(String),
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Unary(Op, Box<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Node {
    VarDecl {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Function {
        name: String,
        params: Vec<(String, Option<Expr>)>,
        body: Vec<Node>,
    },
    Return(Option<Expr>),
    Loop {
        times: Expr,
        body: Vec<Node>,
    },
    While {
        cond: Expr,
        body: Vec<Node>,
    },
    If {
        cond: Expr,
        then_body: Vec<Node>,
        else_body: Option<Vec<Node>>,
    },
    Async {
        body: Vec<Node>,
    },
    Await {
        key: String,
        negated: bool,
        body: Vec<Node>,
    },
    Break,
    Continue,
}

#[derive(Clone, Copy, Debug)]
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