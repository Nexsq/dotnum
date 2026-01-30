use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::ast::{Expr, Node, Op};

#[derive(Clone, Debug)]
pub enum Value {
    Num(i64),
    Str(String),
    Bool(bool),
    Symbol(String),
    Error(String),
}

#[derive(Clone)]
pub struct Context {
    vars: Arc<Mutex<HashMap<String, Value>>>,
    funcs: Arc<Mutex<HashMap<String, Node>>>,
    cmds: Arc<HashMap<String, fn(Vec<Value>) -> Value>>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
    error: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
enum Flow {
    None,
    Break,
    Continue,
    Return(Value),
}

impl Context {
    pub fn new(cmds: HashMap<String, fn(Vec<Value>) -> Value>) -> Self {
        Self {
            vars: Arc::new(Mutex::new(HashMap::new())),
            funcs: Arc::new(Mutex::new(HashMap::new())),
            cmds: Arc::new(cmds),
            tasks: Arc::new(Mutex::new(Vec::new())),
            error: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run(&self, nodes: &[Node]) -> Result<(), String> {
        for n in nodes.iter() {
            if let Some(err) = self.error.lock().unwrap().clone() {
                return Err(err);
            }

            match self.exec(n)? {
                Flow::None => {}
                Flow::Break => return Err("break outside loop".into()),
                Flow::Continue => return Err("continue outside loop".into()),
                Flow::Return(_) => return Err("return outside function".into()),
            }
        }
        Ok(())
    }

    pub fn join_tasks(&self) -> Result<(), String> {
        loop {
            let handle = {
                let mut tasks = self.tasks.lock().unwrap();
                tasks.pop()
            };

            match handle {
                Some(h) => {
                    let _ = h.join();
                }
                None => break,
            }
        }

        if let Some(err) = self.error.lock().unwrap().clone() {
            return Err(err);
        }

        Ok(())
    }

    fn exec(&self, n: &Node) -> Result<Flow, String> {
        if self.error.lock().unwrap().is_some() {
            return Ok(Flow::None);
        }

        match n {
            Node::VarDecl { name, value }
            | Node::Assign { name, value } => {
                let v = self.eval(value)?;
                self.vars.lock().unwrap().insert(name.clone(), v);
            }

            Node::Call { name, args } => {
                let vals = args
                    .iter()
                    .map(|a| self.eval(a))
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(cmd) = self.cmds.get(name) {
                    let result = cmd(vals);
                    if let Value::Error(e) = result {
                        *self.error.lock().unwrap() = Some(e);
                    }
                } else {
                    let result = self.eval(&Expr::Call {
                        name: name.clone(),
                        args: args.clone(),
                    })?;

                    if let Value::Error(e) = result {
                        *self.error.lock().unwrap() = Some(e);
                    }
                }
            }

            Node::Function { name, .. } => {
                self.funcs.lock().unwrap().insert(name.clone(), n.clone());
            }

            Node::Return(expr) => {
                let v = match expr {
                    Some(e) => self.eval(e)?,
                    None => Value::Bool(false),
                };
                return Ok(Flow::Return(v));
            }

            Node::Async { body } => {
                let ctx = self.clone();
                let body = body.clone();

                let handle = std::thread::spawn(move || {
                    if let Err(e) = ctx.run(&body) {
                        *ctx.error.lock().unwrap() = Some(e);
                    }
                });

                self.tasks.lock().unwrap().push(handle);
            }

            Node::Await { key, body } => {
                let device = DeviceState::new();

                loop {
                    let keys = device.get_keys();
                    let mouse = device.get_mouse().button_pressed;

                    let matched = match key.as_str() {
                        "LMB" => mouse[1],
                        "RMB" => mouse[2],
                        "MMB" => mouse[3],
                        "MB4" => mouse[4],
                        "MB5" => mouse[5],

                        "Enter" | "Return" => keys.contains(&Keycode::Enter),
                        "Space" => keys.contains(&Keycode::Space),
                        "Tab" => keys.contains(&Keycode::Tab),
                        "Esc" | "Escape" => keys.contains(&Keycode::Escape),
                        "Backspace" => keys.contains(&Keycode::Backspace),

                        "Ctrl" | "Control" => keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl),
                        "LCtrl" | "LControl" => keys.contains(&Keycode::LControl),
                        "RCtrl" | "RControl" => keys.contains(&Keycode::RControl),
                        "Alt" => keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt),
                        "LAlt" => keys.contains(&Keycode::LAlt),
                        "RAlt" | "AltGr" => keys.contains(&Keycode::RAlt),
                        "Shift" => keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift),
                        "LShift" => keys.contains(&Keycode::LShift),
                        "RShift" => keys.contains(&Keycode::RShift),
                        "Super" | "Meta" => keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta),
                        "CapsLock" | "Caps" => keys.contains(&Keycode::CapsLock),

                        "Insert" => keys.contains(&Keycode::Insert),
                        "Delete" | "Del" => keys.contains(&Keycode::Delete),
                        "Home" => keys.contains(&Keycode::Home),
                        "End" => keys.contains(&Keycode::End),
                        "PageUp" | "PgUp" => keys.contains(&Keycode::PageUp),
                        "PageDown" | "PgDown" => keys.contains(&Keycode::PageDown),

                        "Up" => keys.contains(&Keycode::Up),
                        "Down" => keys.contains(&Keycode::Down),
                        "Left" => keys.contains(&Keycode::Left),
                        "Right" => keys.contains(&Keycode::Right),

                        k if k.starts_with('F') => match k {
                            "F1" => keys.contains(&Keycode::F1),
                            "F2" => keys.contains(&Keycode::F2),
                            "F3" => keys.contains(&Keycode::F3),
                            "F4" => keys.contains(&Keycode::F4),
                            "F5" => keys.contains(&Keycode::F5),
                            "F6" => keys.contains(&Keycode::F6),
                            "F7" => keys.contains(&Keycode::F7),
                            "F8" => keys.contains(&Keycode::F8),
                            "F9" => keys.contains(&Keycode::F9),
                            "F10" => keys.contains(&Keycode::F10),
                            "F11" => keys.contains(&Keycode::F11),
                            "F12" => keys.contains(&Keycode::F12),
                            _ => false,
                        },

                        k if k.len() == 1 && k.chars().next().unwrap().is_ascii_digit() => {
                            let d = k.chars().next().unwrap();
                            keys.iter().any(|kc| match kc {
                                Keycode::Key0 => d == '0',
                                Keycode::Key1 => d == '1',
                                Keycode::Key2 => d == '2',
                                Keycode::Key3 => d == '3',
                                Keycode::Key4 => d == '4',
                                Keycode::Key5 => d == '5',
                                Keycode::Key6 => d == '6',
                                Keycode::Key7 => d == '7',
                                Keycode::Key8 => d == '8',
                                Keycode::Key9 => d == '9',
                                _ => false,
                            })
                        },

                        k if k.len() == 1 => {
                            let c = k.chars().next().unwrap().to_ascii_lowercase();
                            keys.iter().any(|kc| match kc {
                                Keycode::A => c == 'a',
                                Keycode::B => c == 'b',
                                Keycode::C => c == 'c',
                                Keycode::D => c == 'd',
                                Keycode::E => c == 'e',
                                Keycode::F => c == 'f',
                                Keycode::G => c == 'g',
                                Keycode::H => c == 'h',
                                Keycode::I => c == 'i',
                                Keycode::J => c == 'j',
                                Keycode::K => c == 'k',
                                Keycode::L => c == 'l',
                                Keycode::M => c == 'm',
                                Keycode::N => c == 'n',
                                Keycode::O => c == 'o',
                                Keycode::P => c == 'p',
                                Keycode::Q => c == 'q',
                                Keycode::R => c == 'r',
                                Keycode::S => c == 's',
                                Keycode::T => c == 't',
                                Keycode::U => c == 'u',
                                Keycode::V => c == 'v',
                                Keycode::W => c == 'w',
                                Keycode::X => c == 'x',
                                Keycode::Y => c == 'y',
                                Keycode::Z => c == 'z',
                                _ => false,
                            })
                        }

                        _ => false,
                    };

                    if matched {
                        for stmt in body.iter() {
                            match self.exec(stmt)? {
                                Flow::None => {}
                                f => return Ok(f),
                            }
                        }
                        break;
                    }

                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }

            Node::Break => return Ok(Flow::Break),
            Node::Continue => return Ok(Flow::Continue),

            Node::Loop { times, body } => {
                let count = match self.eval(times)? {
                    Value::Num(n) => n,
                    _ => return Err("loop expects number".into()),
                };

                for _ in 0..count {
                    for stmt in body.iter() {
                        match self.exec(stmt)? {
                            Flow::None => {}
                            Flow::Continue => break,
                            Flow::Break => return Ok(Flow::None),
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                        }
                    }
                }
            }

            Node::While { cond, body } => {
                while matches!(self.eval(cond)?, Value::Bool(true)) {
                    for stmt in body.iter() {
                        match self.exec(stmt)? {
                            Flow::None => {}
                            Flow::Continue => break,
                            Flow::Break => return Ok(Flow::None),
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                        }
                    }
                }
            }

            Node::If { cond, then_body, else_body } => {
                let cond = self.eval(cond)?;
                let branch = match cond {
                    Value::Bool(true) => Some(then_body),
                    Value::Bool(false) => else_body.as_ref(),
                    _ => return Err("if condition must be boolean".into()),
                };

                if let Some(stmts) = branch {
                    for stmt in stmts.iter() {
                        match self.exec(stmt)? {
                            Flow::None => {}
                            f => return Ok(f),
                        }
                    }
                }
            }
        }

        Ok(Flow::None)
    }

    fn eval(&self, e: &Expr) -> Result<Value, String> {
        match e {
            Expr::Number(n) => Ok(Value::Num(*n)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Var(name) => {
                if let Some(v) = self.vars.lock().unwrap().get(name).cloned() {
                    Ok(v)
                } else {
                    Ok(Value::Symbol(name.clone()))
                }
            }

            Expr::Call { name, args } => {
                let vals = args
                    .iter()
                    .map(|a| self.eval(a))
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(cmd) = self.cmds.get(name) {
                    Ok(cmd(vals))
                } else {
                    let f = self.funcs.lock().unwrap().get(name).cloned()
                        .ok_or(format!("undefined function {}", name))?;

                    let (params, body) = match f {
                        Node::Function { params, body, .. } => (params, body),
                        _ => unreachable!(),
                    };

                    let mut locals = HashMap::new();
                    for (i, (p, def)) in params.iter().enumerate() {
                        let v = if let Some(a) = args.get(i) {
                            self.eval(a)?
                        } else if let Some(d) = def {
                            self.eval(d)?
                        } else {
                            return Err(format!("missing argument {}", p));
                        };
                        locals.insert(p.clone(), v);
                    }

                    let saved = self.vars.lock().unwrap().clone();
                    *self.vars.lock().unwrap() = locals;

                    let mut ret = Value::Bool(false);
                    for stmt in body.iter() {
                        match self.exec(stmt)? {
                            Flow::Return(v) => {
                                ret = v;
                                break;
                            }
                            Flow::None => {}
                            _ => return Err("invalid control flow in function".into()),
                        }
                    }

                    *self.vars.lock().unwrap() = saved;
                    Ok(ret)
                }
            }

            Expr::Unary(op, e) => {
                let v = self.eval(e)?;
                match (op, v) {
                    (Op::Sub, Value::Num(n)) => Ok(Value::Num(-n)),
                    (Op::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err("invalid unary op".into()),
                }
            }

            Expr::Binary(a, op, b) => {
                let l = self.eval(a)?;
                let r = self.eval(b)?;

                match (l, r, op) {
                    (Value::Num(x), Value::Num(y), Op::Add) => Ok(Value::Num(x + y)),
                    (Value::Str(x), y, Op::Add) => Ok(Value::Str(x + &format_value(&y))),
                    (x, Value::Str(y), Op::Add) => Ok(Value::Str(format_value(&x) + &y)),
                    (Value::Num(x), Value::Num(y), Op::Sub) => Ok(Value::Num(x - y)),
                    (Value::Num(x), Value::Num(y), Op::Mul) => Ok(Value::Num(x * y)),
                    (Value::Num(x), Value::Num(y), Op::Div) => Ok(Value::Num(x / y)),
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
                    _ => Err("type error".into()),
                }
            }
        }
    }
}

fn format_value(v: &Value) -> String {
    match v {
        Value::Num(n) => n.to_string(),
        Value::Str(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Symbol(s) => s.clone(),
        Value::Error(e) => e.clone(),
    }
}