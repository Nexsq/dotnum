use crate::interpreter::Value;
use std::collections::HashMap;

pub fn expect_arity(name: &str, args: &[Value], expected: usize) -> Result<(), Value> {
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

pub mod abs;
pub mod background;
pub mod beep;
pub mod click;
pub mod color;
pub mod exit;
pub mod get_color;
pub mod get_mouse;
pub mod get_resolution;
pub mod mouse;
pub mod press;
pub mod print;
pub mod process;
pub mod random;
pub mod release;
pub mod scroll;
pub mod sleep;
pub mod string;
pub mod time;

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
    get_mouse::register(map);
    beep::register(map);
    background::register(map);
    get_resolution::register(map);
    color::register(map);
    get_color::register(map);
    process::register(map);
    abs::register(map);
}
