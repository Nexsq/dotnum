use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::collections::HashMap;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("mouse".into(), mouse);
}

fn mouse(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("mouse", &args, 3) {
        return e;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let x = match args.get(0) {
        Some(Value::Num(n)) => *n as i32,
        _ => return Value::Error("mouse expects number x".into()),
    };

    let y = match args.get(1) {
        Some(Value::Num(n)) => *n as i32,
        _ => return Value::Error("mouse expects number y".into()),
    };

    let coord = match args.get(2) {
        Some(Value::Symbol(s)) | Some(Value::Str(s)) => match s.as_str() {
            "abs" | "absolute" => Coordinate::Abs,
            "rel" | "relative" => Coordinate::Rel,
            _ => return Value::Error("mouse expects abs or rel".into()),
        },
        _ => return Value::Error("mouse expects coordinate mode".into()),
    };

    let _ = enigo.move_mouse(x, y, coord);
    Value::Bool(false)
}
