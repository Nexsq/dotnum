use super::BuiltinFn;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("print".into(), print);
}

fn print(args: Vec<Value>) -> Value {
    for v in args {
        match v {
            Value::Num(n) => print!("{}", n),
            Value::Str(s) => print!("{}", s),
            Value::Bool(b) => print!("{}", b),
            Value::Symbol(s) => print!("{}", s),
            Value::Error(e) => eprint!("Error: {}", e),
        }
    }
    println!();
    Value::Bool(false)
}
