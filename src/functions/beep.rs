use crate::interpreter::Value;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn Beep(freq: u32, duration: u32) -> i32;
}

pub fn register(map: &mut HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("beep".into(), beep);
}

fn beep(args: Vec<Value>) -> Value {
    let pitch = match args.get(0) {
        Some(Value::Num(n)) => *n,
        _ => 440,
    };

    let freq = pitch.clamp(37, 32767) as u32;

    #[cfg(target_os = "windows")]
    unsafe {
        Beep(freq, 200);
    }

    Value::Bool(true)
}