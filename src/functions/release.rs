use enigo::{Button, Direction::Release, Enigo, Key, Keyboard, Mouse, Settings};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

static ENIGO: OnceLock<Mutex<Enigo>> = OnceLock::new();

fn with_enigo<F, R>(f: F) -> R
where
    F: FnOnce(&mut Enigo) -> R,
{
    let enigo = ENIGO.get_or_init(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));
    let mut enigo = enigo.lock().unwrap();
    f(&mut enigo)
}

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("release".into(), release);
}

fn release(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("release", &args, 1) {
        return e;
    }

    let sym_owned;
    let sym = match args.get(0) {
        Some(Value::Key(s)) => s.as_str(),
        Some(Value::Str(s)) => s.as_str(),
        Some(Value::Num(n)) => {
            sym_owned = n.to_string();
            &sym_owned
        }
        _ => return Value::Error("release expects a key".into()),
    };

    with_enigo(|enigo| {
        match sym {
            "LMB" => {
                let _ = enigo.button(Button::Left, Release);
            }
            "MMB" => {
                let _ = enigo.button(Button::Middle, Release);
            }
            "RMB" => {
                let _ = enigo.button(Button::Right, Release);
            }
            "MB4" => {
                let _ = enigo.button(Button::Back, Release);
            }
            "MB5" => {
                let _ = enigo.button(Button::Forward, Release);
            }

            s if s.len() == 1 && s.chars().next().unwrap().is_ascii_digit() => {
                let k = match s {
                    "0" => Key::Num0,
                    "1" => Key::Num1,
                    "2" => Key::Num2,
                    "3" => Key::Num3,
                    "4" => Key::Num4,
                    "5" => Key::Num5,
                    "6" => Key::Num6,
                    "7" => Key::Num7,
                    "8" => Key::Num8,
                    "9" => Key::Num9,
                    _ => unreachable!(),
                };
                let _ = enigo.key(k, Release);
            }

            s if s.len() == 1 => {
                let c = s.chars().next().unwrap();
                let _ = enigo.key(Key::Unicode(c), Release);
            }

            "Enter" | "Return" => {
                let _ = enigo.key(Key::Return, Release);
            }
            "Space" => {
                let _ = enigo.key(Key::Space, Release);
            }
            "Tab" => {
                let _ = enigo.key(Key::Tab, Release);
            }
            "Esc" | "Escape" => {
                let _ = enigo.key(Key::Escape, Release);
            }
            "Backspace" => {
                let _ = enigo.key(Key::Backspace, Release);
            }

            "Ctrl" | "Control" => {
                let _ = enigo.key(Key::Control, Release);
            }
            "Alt" => {
                let _ = enigo.key(Key::Alt, Release);
            }
            "Shift" => {
                let _ = enigo.key(Key::Shift, Release);
            }
            "Super" | "Meta" => {
                let _ = enigo.key(Key::Meta, Release);
            }
            "CapsLock" | "Caps" => {
                let _ = enigo.key(Key::CapsLock, Release);
            }

            "Insert" => {
                let _ = enigo.key(Key::Insert, Release);
            }
            "Delete" | "Del" => {
                let _ = enigo.key(Key::Delete, Release);
            }
            "Home" => {
                let _ = enigo.key(Key::Home, Release);
            }
            "End" => {
                let _ = enigo.key(Key::End, Release);
            }
            "PageUp" | "PgUp" => {
                let _ = enigo.key(Key::PageUp, Release);
            }
            "PageDown" | "PgDown" => {
                let _ = enigo.key(Key::PageDown, Release);
            }

            "Up" => {
                let _ = enigo.key(Key::UpArrow, Release);
            }
            "Down" => {
                let _ = enigo.key(Key::DownArrow, Release);
            }
            "Left" => {
                let _ = enigo.key(Key::LeftArrow, Release);
            }
            "Right" => {
                let _ = enigo.key(Key::RightArrow, Release);
            }

            "F1" => {
                let _ = enigo.key(Key::F1, Release);
            }
            "F2" => {
                let _ = enigo.key(Key::F2, Release);
            }
            "F3" => {
                let _ = enigo.key(Key::F3, Release);
            }
            "F4" => {
                let _ = enigo.key(Key::F4, Release);
            }
            "F5" => {
                let _ = enigo.key(Key::F5, Release);
            }
            "F6" => {
                let _ = enigo.key(Key::F6, Release);
            }
            "F7" => {
                let _ = enigo.key(Key::F7, Release);
            }
            "F8" => {
                let _ = enigo.key(Key::F8, Release);
            }
            "F9" => {
                let _ = enigo.key(Key::F9, Release);
            }
            "F10" => {
                let _ = enigo.key(Key::F10, Release);
            }
            "F11" => {
                let _ = enigo.key(Key::F11, Release);
            }
            "F12" => {
                let _ = enigo.key(Key::F12, Release);
            }

            _ => return Value::Error("invalid release target".into()),
        }
        Value::Bool(false)
    })
}
