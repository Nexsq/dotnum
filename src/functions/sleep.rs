use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value};
use std::{collections::HashMap, thread, time::Duration};

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("sleep".into(), sleep);
}

fn sleep(_ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("sleep", &args, 1) {
        return e;
    }

    let ms = match args.get(0) {
        Some(Value::Num(n)) => *n,
        _ => return Value::Bool(false),
    };

    if ms > 0 {
        thread::sleep(Duration::from_millis(ms as u64));
    }

    Value::Bool(false)
}
