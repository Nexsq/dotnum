use crate::interpreter::Value;
use std::process;

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("exit".into(), exit);
}

fn exit(_args: Vec<Value>) -> Value {
    process::exit(0);
}