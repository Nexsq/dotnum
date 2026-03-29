use super::BuiltinFn;
use crate::interpreter::{Context, Value};
use std::collections::HashMap;
use std::process;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("exit".into(), exit);
}

fn exit(_ctx: &Context, _args: Vec<Value>) -> Value {
    process::exit(0);
}
