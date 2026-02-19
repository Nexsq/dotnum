use crate::functions::expect_arity;
use crate::interpreter::Value;
use device_query::{DeviceQuery, DeviceState};

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("get_mouse".into(), get_mouse);
}

fn get_mouse(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("get_mouse", &args, 1) {
        return e;
    }

    let axis = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.as_str(),
        _ => return Value::Error("get_mouse expects axis name".into()),
    };

    let device = DeviceState::new();
    let (x, y) = device.get_mouse().coords;

    let value = match axis {
        "x" | "hor" | "width" => x as i64,
        "y" | "ver" | "height" => y as i64,
        _ => return Value::Error("invalid mouse axis".into()),
    };

    Value::Num(value)
}
