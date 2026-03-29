use enigo::{Axis, Mouse};
use std::collections::HashMap;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value};

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("scroll".into(), scroll);
}

fn scroll(ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("scroll", &args, 2) {
        return e;
    }

    let mut enigo = ctx.enigo.lock().unwrap();

    let amount = match args.get(0) {
        Some(Value::Num(n)) => *n as i32,
        _ => return Value::Error("scroll expects number amount".into()),
    };

    let axis = match args.get(1) {
        Some(Value::Symbol(s)) | Some(Value::Str(s)) => match s.as_str() {
            "ver" | "vertical" | "v" => Axis::Vertical,
            "hor" | "horizontal" | "h" => Axis::Horizontal,
            _ => return Value::Error("scroll expects ver/hor".into()),
        },
        _ => return Value::Error("scroll expects axis name".into()),
    };

    let _ = enigo.scroll(amount, axis);
    Value::Bool(true)
}
