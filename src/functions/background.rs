use super::BuiltinFn;
use crate::interpreter::{Context, Value};
use std::collections::HashMap;

#[cfg(windows)]
use winapi::um::wincon::FreeConsole;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("background".into(), background);
}

fn background(_ctx: &Context, _args: Vec<Value>) -> Value {
    #[cfg(windows)]
    unsafe {
        FreeConsole();
        Value::Bool(false)
    }

    #[cfg(not(windows))]
    {
        Value::Error("background is Windows-only".into())
    }
}
