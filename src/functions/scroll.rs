use std::collections::HashMap;
use enigo::{
    Axis,
    Enigo, Settings,
    Mouse,
};

use crate::interpreter::Value;
use crate::functions::expect_arity;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("scroll".into(), scroll);
}

fn scroll(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("scroll", &args, 2) {
        return e;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let amount = match args.get(0) {
        Some(Value::Num(n)) => *n as i32,
        _ => return Value::Bool(false),
    };

    let axis = match args.get(1) {
        Some(Value::Symbol(s)) => match s.as_str() {
            "ver" | "vertical" => Axis::Vertical,
            "hor" | "horizontal" => Axis::Horizontal,
            _ => return Value::Bool(false),
        },
        _ => return Value::Bool(false),
    };

    let _ = enigo.scroll(amount, axis);

    Value::Bool(false)
}