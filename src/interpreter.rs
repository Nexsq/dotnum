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

enum Flow {
    None,
    Break,
    Continue,
}

impl Context {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn run_with_commands(
        &mut self,
        nodes: &[Node],
        cmds: &HashMap<String, fn(&mut Context, Vec<Value>)>,
    ) -> Result<(), String> {
        for n in nodes {
            match self.exec(n, cmds)? {
                Flow::None => {}
                Flow::Break => return Err("break outside loop".into()),
                Flow::Continue => return Err("continue outside loop".into()),
            }
        }
        Ok(())
    }

    fn exec(
        &mut self,
        n: &Node,
        cmds: &HashMap<String, fn(&mut Context, Vec<Value>)>,
    ) -> Result<Flow, String> {
        match n {
            Node::VarDecl { name, value } => {
                let v = self.eval(value)?;
                self.vars.insert(name.clone(), v);
            }

            Node::Assign { name, value } => {
                let v = self.eval(value)?;
                self.vars.insert(name.clone(), v);
            }

            Node::Call { name, args } => {
                let vals: Vec<_> = args.iter().map(|a| self.eval(a)).collect::<Result<_,_>>()?;
                if let Some(cmd) = cmds.get(name) {
                    cmd(self, vals);
                } else {
                    return Err(format!("Unknown command '{}'", name));
                }
            }

            Node::Loop { times, body } => {
                if let Value::Num(n) = self.eval(times)? {
                    for _ in 0..n {
                        for stmt in body {
                            match self.exec(stmt, cmds)? {
                                Flow::None => {}
                                Flow::Continue => break,
                                Flow::Break => return Ok(Flow::None),
                            }
                        }
                    }
                } else {
                    return Err("Loop expects number".into());
                }
            }

            Node::While { cond, body } => {
                while let Value::Bool(true) = self.eval(cond)? {
                    for stmt in body {
                        match self.exec(stmt, cmds)? {
                            Flow::None => {}
                            Flow::Continue => break,
                            Flow::Break => return Ok(Flow::None),
                        }
                    }
                }
            }

            Node::If { cond, then_body, else_body } => {
                if let Value::Bool(b) = self.eval(cond)? {
                    if b {
                        for stmt in then_body {
                            match self.exec(stmt, cmds)? {
                                Flow::None => {}
                                f => return Ok(f),
                            }
                        }
                    } else if let Some(e) = else_body {
                        for stmt in e {
                            match self.exec(stmt, cmds)? {
                                Flow::None => {}
                                f => return Ok(f),
                            }
                        }
                    }
                } else {
                    return Err("If condition must be boolean".into());
                }
            }

            Node::Break => return Ok(Flow::Break),
            Node::Continue => return Ok(Flow::Continue),
        }

        Ok(Flow::None)
    }

    fn eval(&mut self, e: &Expr) -> Result<Value, String> {
        match e {
            Expr::Number(n) => Ok(Value::Num(*n)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Var(name) => self.vars
                .get(name)
                .cloned()
                .ok_or(format!("Undefined variable {}", name)),

            Expr::Unary(op, e) => {
                let v = self.eval(e)?;
                match (op, v) {
                    (Op::Sub, Value::Num(n)) => Ok(Value::Num(-n)),
                    (Op::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err("Invalid unary op".into()),
                }
            }

            Expr::Binary(a, op, b) => {
                let l = self.eval(a)?;
                let r = self.eval(b)?;
                match (l, r, op) {
                    (Value::Num(x), Value::Num(y), Op::Add) => Ok(Value::Num(x + y)),
                    (Value::Num(x), Value::Num(y), Op::Sub) => Ok(Value::Num(x - y)),
                    (Value::Num(x), Value::Num(y), Op::Mul) => Ok(Value::Num(x * y)),
                    (Value::Num(x), Value::Num(y), Op::Div) => Ok(Value::Num(x / y)),

                    (Value::Str(a), Value::Str(b), Op::Add) => Ok(Value::Str(a + &b)),
                    (Value::Str(a), Value::Num(b), Op::Add) => Ok(Value::Str(a + &b.to_string())),
                    (Value::Num(a), Value::Str(b), Op::Add) => Ok(Value::Str(a.to_string() + &b)),
                    (Value::Str(a), Value::Bool(b), Op::Add) => Ok(Value::Str(a + &b.to_string())),
                    (Value::Bool(a), Value::Str(b), Op::Add) => Ok(Value::Str(a.to_string() + &b)),

                    (Value::Num(x), Value::Num(y), Op::Eq) => Ok(Value::Bool(x == y)),
                    (Value::Num(x), Value::Num(y), Op::Ne) => Ok(Value::Bool(x != y)),
                    (Value::Num(x), Value::Num(y), Op::Gt) => Ok(Value::Bool(x > y)),
                    (Value::Num(x), Value::Num(y), Op::Lt) => Ok(Value::Bool(x < y)),
                    (Value::Num(x), Value::Num(y), Op::Ge) => Ok(Value::Bool(x >= y)),
                    (Value::Num(x), Value::Num(y), Op::Le) => Ok(Value::Bool(x <= y)),

                    (Value::Bool(x), Value::Bool(y), Op::Eq) => Ok(Value::Bool(x == y)),
                    (Value::Bool(x), Value::Bool(y), Op::Ne) => Ok(Value::Bool(x != y)),
                    (Value::Bool(x), Value::Bool(y), Op::And) => Ok(Value::Bool(x && y)),
                    (Value::Bool(x), Value::Bool(y), Op::Or) => Ok(Value::Bool(x || y)),

                    _ => Err("Type error".into()),
                }
            }
        }
    }
}