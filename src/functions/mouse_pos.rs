use crate::interpreter::Value;
use device_query::{DeviceQuery, DeviceState};

pub fn register(
    map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>
) {
    map.insert("mouse_pos".into(), mouse_pos);
}

fn mouse_pos(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        return Value::Error("mouse_pos expects 'x' or 'y'".into());
    }

    let axis = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.as_str(),
        _ => return Value::Error("mouse_pos expects 'x' or 'y'".into()),
    };

    let device = DeviceState::new();
    let (x, y) = device.get_mouse().coords;

    match axis {
        "x" => Value::Num(x as i64),
        "y" => Value::Num(y as i64),
        _ => Value::Error("mouse_pos expects 'x' or 'y'".into()),
    }
}