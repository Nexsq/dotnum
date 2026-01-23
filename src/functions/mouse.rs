use std::collections::HashMap;
use enigo::{
    Coordinate,
    Enigo, Settings,
    Mouse,
};

use crate::interpreter::Value;
use crate::functions::expect_arity;
use super::BuiltinFn;

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
        _ => return Value::Bool(false),
    };

    let y = match args.get(1) {
        Some(Value::Num(n)) => *n as i32,
        _ => return Value::Bool(false),
    };

    let coord = match args.get(2) {
        Some(Value::Symbol(s)) => match s.as_str() {
            "abs" | "absolute" => Coordinate::Abs,
            "rel" | "relative" => Coordinate::Rel,
            _ => return Value::Bool(false),
        },
        _ => return Value::Bool(false),
    };

    let _ = enigo.move_mouse(x, y, coord);

    Value::Bool(false)
}