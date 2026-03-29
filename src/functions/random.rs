use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value};
use rand::Rng;
use std::collections::HashMap;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("random".into(), random);
}

fn random(_ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("random", &args, 2) {
        return e;
    }

    let (a, b) = match (&args[0], &args[1]) {
        (Value::Num(x), Value::Num(y)) => (*x, *y),
        _ => return Value::Error("random expects numbers".into()),
    };

    if a > b {
        return Value::Error("random range is invalid".into());
    }

    let mut rng = rand::rng();
    let n = rng.random_range(a..=b);
    Value::Num(n)
}
