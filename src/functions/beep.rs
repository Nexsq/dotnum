use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn Beep(freq: u32, duration: u32) -> i32;
}

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("beep".into(), beep);
}

fn beep(_ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("beep", &args, 1) {
        return e;
    }

    let pitch = match args.get(0) {
        Some(Value::Num(n)) => *n,
        _ => 440,
    };

    let freq = pitch.clamp(37, 32767) as u32;

    #[cfg(target_os = "windows")]
    unsafe {
        Beep(freq, 200);
        Value::Bool(true)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Value::Error("beep is only supported on Windows".into())
    }
}
