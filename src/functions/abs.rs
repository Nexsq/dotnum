use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value}; 
use std::collections::HashMap;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("abs".into(), abs);
}

fn abs(_ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("abs", &args, 1) {
        return e;
    }

    match &args[0] {
        Value::Num(n) => Value::Num(n.abs()),
        _ => Value::Error("abs expects a number".into()),
    }
}
