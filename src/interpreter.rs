use std::collections::HashMap;
use crate::ast::{Expr, Node, Op};

#[derive(Clone, Debug)]
pub enum Value {
    Num(i64),
    Str(String),
    Bool(bool),
}

pub struct Context {
    vars: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn run_with_commands(&mut self, nodes: &[Node], cmds: &HashMap<String, fn(&mut Context, Vec<Value>)>) {
        for n in nodes {
            self.exec(n, cmds);
        }
    }

    fn exec(&mut self, n: &Node, cmds: &HashMap<String, fn(&mut Context, Vec<Value>)>) {
        match n {
            Node::VarDecl { name, value } => {
                let v = self.eval(value);
                self.vars.insert(name.clone(), v);
            }
            Node::Assign { name, value } => {
                let v = self.eval(value);
                self.vars.insert(name.clone(), v);
            }
            Node::Call { name, args } => {
                let vals: Vec<_> = args.iter().map(|a| self.eval(a)).collect();
                if let Some(cmd) = cmds.get(name) {
                    cmd(self, vals);
                } else {
                    panic!("Unknown command '{}'", name);
                }
            }
            Node::Loop { times, body } => {
                if let Value::Num(n) = self.eval(times) {
                    for _ in 0..n {
                        self.run_with_commands(body, cmds);
                    }
                }
            }
            Node::If { cond, then_body, else_body } => {
                if let Value::Bool(b) = self.eval(cond) {
                    if b {
                        self.run_with_commands(then_body, cmds)
                    } else if let Some(e) = else_body {
                        self.run_with_commands(e, cmds)
                    }
                }
            }
        }
    }

    fn eval(&mut self, e: &Expr) -> Value {
        match e {
            Expr::Number(n) => Value::Num(*n),
            Expr::Str(s) => Value::Str(s.clone()),
            Expr::Var(name) => self.vars.get(name).cloned().unwrap(),
            Expr::Binary(a, op, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                match (l, r, op) {
                    (Value::Num(x), Value::Num(y), Op::Eq) => Value::Bool(x == y),
                    (Value::Num(x), Value::Num(y), Op::Ne) => Value::Bool(x != y),
                    (Value::Num(x), Value::Num(y), Op::Gt) => Value::Bool(x > y),
                    (Value::Num(x), Value::Num(y), Op::Lt) => Value::Bool(x < y),
                    (Value::Num(x), Value::Num(y), Op::Ge) => Value::Bool(x >= y),
                    (Value::Num(x), Value::Num(y), Op::Le) => Value::Bool(x <= y),
                    (Value::Bool(x), Value::Bool(y), Op::And) => Value::Bool(x && y),
                    (Value::Bool(x), Value::Bool(y), Op::Or) => Value::Bool(x || y),
                    _ => panic!("Type error"),
                }
            }
        }
    }
}