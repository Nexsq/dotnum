use std::collections::HashMap;
use sysinfo::{ProcessesToUpdate, System};

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("process".into(), process);
}

fn process(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("process", &args, 1) {
        return e;
    }

    let target = match &args[0] {
        Value::Str(s) | Value::Symbol(s) => s.to_lowercase(),
        _ => return Value::Error("process expects process name string".into()),
    };

    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    for process in sys.processes().values() {
        if process.name().to_string_lossy().to_lowercase() == target {
            return Value::Bool(true);
        }
    }

    Value::Bool(false)
}
