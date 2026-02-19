use enigo::{Enigo, Keyboard, Settings};
use std::collections::HashMap;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("string".into(), string);
}

fn string(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("string", &args, 1) {
        return e;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let text = match args.get(0) {
        Some(Value::Str(s)) | Some(Value::Symbol(s)) => s,
        _ => return Value::Error("string expects text".into()),
    };

    let _ = enigo.text(text);
    Value::Bool(false)
}
