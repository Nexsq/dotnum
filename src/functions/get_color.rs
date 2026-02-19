use scrap::{Capturer, Display};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::ErrorKind;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

struct CaptureState {
    capturer: Capturer,
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

thread_local! {
    static STATE: RefCell<Option<CaptureState>> = RefCell::new(None);
}

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("get_color".into(), get_color);
}

fn with_state<F>(f: F) -> Option<Value>
where
    F: FnOnce(&mut CaptureState) -> Value,
{
    STATE.with(|cell| {
        let mut opt = cell.borrow_mut();
        if opt.is_none() {
            let display = Display::primary().ok()?;
            let capturer = Capturer::new(display).ok()?;
            let width = capturer.width();
            let height = capturer.height();
            *opt = Some(CaptureState {
                capturer,
                width,
                height,
                buffer: vec![0; width * height * 4],
            });
        }
        let state = opt.as_mut()?;
        Some(f(state))
    })
}

fn get_color(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("get_color", &args, 2) {
        return e;
    }

    let x = match args.get(0) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("get_color expects number x".into()),
    };

    let y = match args.get(1) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("get_color expects number y".into()),
    };

    match with_state(|state| {
        match state.capturer.frame() {
            Ok(frame) => {
                state.buffer.copy_from_slice(&frame);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(_) => return Value::Error("failed to read pixel".into()),
        }

        if x >= state.width || y >= state.height {
            return Value::Error("failed to read pixel".into());
        }

        let stride = state.width * 4;
        let idx = y * stride + x * 4;

        if idx + 2 >= state.buffer.len() {
            return Value::Error("failed to read pixel".into());
        }

        let b = state.buffer[idx];
        let g = state.buffer[idx + 1];
        let r = state.buffer[idx + 2];

        Value::Str(format!("#{:02x}{:02x}{:02x}", r, g, b))
    }) {
        Some(v) => v,
        None => Value::Error("failed to read pixel".into()),
    }
}
