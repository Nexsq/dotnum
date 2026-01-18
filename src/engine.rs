use std::collections::HashMap;
use crate::{interpreter::{Context, Value}, ast::Node};

pub type CommandFn = fn(&mut Context, Vec<Value>);

pub struct Engine {
    ctx: Context,
    commands: HashMap<String, CommandFn>,
}

impl Engine {
    pub fn new() -> Self {
        let mut e = Self {
            ctx: Context::new(),
            commands: HashMap::new(),
        };

        e.register("print", |_, args| {
            for v in args {
                match v {
                    Value::Num(n) => print!("{n} "),
                    Value::Str(s) => print!("{s} "),
                    Value::Bool(b) => print!("{b} "),
                }
            }
            println!();
        });

        e.register("sleep", |_, args| {
            if let Some(Value::Num(ms)) = args.get(0) {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            }
        });

        e
    }

    pub fn register(&mut self, name: &str, f: CommandFn) {
        self.commands.insert(name.to_string(), f);
    }

    pub fn run(&mut self, nodes: Vec<Node>) {
        self.ctx.run_with_commands(&nodes, &self.commands);
    }
}