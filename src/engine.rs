use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::{interpreter::{Context, Value}, ast::Node};

pub type CommandFn = fn(&mut Context, Vec<Value>);

pub struct Engine {
    ctx: Arc<Mutex<Context>>,
    commands: HashMap<String, CommandFn>,
}

impl Engine {
    pub fn new() -> Self {
        let mut e = Self {
            ctx: Arc::new(Mutex::new(Context::new())),
            commands: HashMap::new(),
        };

        crate::commands::register_all(&mut e);

        e
    }

    pub fn register(&mut self, name: &str, f: CommandFn) {
        self.commands.insert(name.to_string(), f);
    }

    pub fn run(&mut self, nodes: Vec<Node>) -> Result<(), String> {
        let mut guard = self.ctx.lock().unwrap();
        guard.run_with_commands(&nodes, &self.commands)
    }
}