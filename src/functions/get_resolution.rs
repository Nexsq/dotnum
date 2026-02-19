use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;
use std::collections::HashMap;

#[cfg(windows)]
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("get_resolution".into(), get_resolution);
}

fn get_resolution(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("get_resolution", &args, 1) {
        return e;
    }

    let axis = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.as_str(),
        _ => return Value::Error("get_resolution expects axis name".into()),
    };

    #[cfg(windows)]
    unsafe {
        match axis {
            "hor" | "x" | "width" => Value::Num(GetSystemMetrics(SM_CXSCREEN) as i64),
            "ver" | "y" | "height" => Value::Num(GetSystemMetrics(SM_CYSCREEN) as i64),
            _ => Value::Error("get_resolution expects hor/ver".into()),
        }
    }

    #[cfg(not(windows))]
    {
        Value::Error("get_resolution is only supported on Windows".into())
    }
}
