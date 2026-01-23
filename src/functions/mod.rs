use std::collections::HashMap;
use crate::interpreter::Value;

pub fn expect_arity(
    name: &str,
    args: &[Value],
    expected: usize,
) -> Result<(), Value> {
    if args.len() < expected {
        Err(Value::Error(format!(
            "{} expects {} argument(s), got {}",
            name,
            expected,
            args.len()
        )))
    } else {
        Ok(())
    }
}

pub mod print;
pub mod sleep;
pub mod click;
pub mod press;
pub mod release;
pub mod scroll;
pub mod mouse;
pub mod string;
pub mod exit;
pub mod time;
pub mod random;
pub mod mouse_pos;
pub mod beep;

pub type BuiltinFn = fn(Vec<Value>) -> Value;

pub fn register_all(map: &mut HashMap<String, BuiltinFn>) {
    print::register(map);
    sleep::register(map);
    click::register(map);
    press::register(map);
    release::register(map);
    scroll::register(map);
    mouse::register(map);
    string::register(map);
    exit::register(map);
    time::register(map);
    random::register(map);
    mouse_pos::register(map);
    beep::register(map);
}