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
    map.insert("color".into(), color);
}

fn parse_hex(s: &str) -> Option<(i64, i64, i64)> {
    let h = s.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = i64::from_str_radix(&h[0..2], 16).ok()?;
    let g = i64::from_str_radix(&h[2..4], 16).ok()?;
    let b = i64::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
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

fn color(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("color", &args, 4) {
        return e;
    }

    let hex = match args.get(0) {
        Some(Value::Symbol(s)) | Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error("color expects hex string".into()),
    };

    let (er, eg, eb) = match parse_hex(hex) {
        Some(v) => v,
        None => return Value::Error("invalid hex color".into()),
    };

    let x = match args.get(1) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("color expects number x".into()),
    };

    let y = match args.get(2) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("color expects number y".into()),
    };

    let tol = match args.get(3) {
        Some(Value::Num(n)) if *n >= 0 => *n,
        _ => return Value::Error("color expects tolerance".into()),
    };

    match with_state(|state| {
        match state.capturer.frame() {
            Ok(frame) => {
                state.buffer.copy_from_slice(&frame);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(_) => return Value::Bool(false),
        }

        if x >= state.width || y >= state.height {
            return Value::Bool(false);
        }

        let stride = state.width * 4;
        let idx = y * stride + x * 4;

        if idx + 2 >= state.buffer.len() {
            return Value::Bool(false);
        }

        let b = state.buffer[idx] as i64;
        let g = state.buffer[idx + 1] as i64;
        let r = state.buffer[idx + 2] as i64;

        let dr = r - er;
        let dg = g - eg;
        let db = b - eb;

        Value::Bool(dr <= tol && dr >= -tol && dg <= tol && dg >= -tol && db <= tol && db >= -tol)
    }) {
        Some(v) => v,
        None => Value::Bool(false),
    }
}
