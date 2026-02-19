use enigo::{Button, Direction::Press, Enigo, Key, Keyboard, Mouse, Settings};
use std::collections::HashMap;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("press".into(), press);
}

fn press(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("press", &args, 1) {
        return e;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) | Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error("press expects key or button name".into()),
    };

    match sym {
        "LMB" => {
            let _ = enigo.button(Button::Left, Press);
        }
        "MMB" => {
            let _ = enigo.button(Button::Middle, Press);
        }
        "RMB" => {
            let _ = enigo.button(Button::Right, Press);
        }
        "MB4" => {
            let _ = enigo.button(Button::Back, Press);
        }
        "MB5" => {
            let _ = enigo.button(Button::Forward, Press);
        }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Press);
        }

        "Enter" | "Return" => {
            let _ = enigo.key(Key::Return, Press);
        }
        "Space" => {
            let _ = enigo.key(Key::Space, Press);
        }
        "Tab" => {
            let _ = enigo.key(Key::Tab, Press);
        }
        "Esc" | "Escape" => {
            let _ = enigo.key(Key::Escape, Press);
        }
        "Backspace" => {
            let _ = enigo.key(Key::Backspace, Press);
        }

        "Ctrl" | "Control" => {
            let _ = enigo.key(Key::Control, Press);
        }
        "Alt" => {
            let _ = enigo.key(Key::Alt, Press);
        }
        "Shift" => {
            let _ = enigo.key(Key::Shift, Press);
        }
        "Super" | "Meta" => {
            let _ = enigo.key(Key::Meta, Press);
        }
        "CapsLock" | "Caps" => {
            let _ = enigo.key(Key::CapsLock, Press);
        }

        "Insert" => {
            let _ = enigo.key(Key::Insert, Press);
        }
        "Delete" | "Del" => {
            let _ = enigo.key(Key::Delete, Press);
        }
        "Home" => {
            let _ = enigo.key(Key::Home, Press);
        }
        "End" => {
            let _ = enigo.key(Key::End, Press);
        }
        "PageUp" | "PgUp" => {
            let _ = enigo.key(Key::PageUp, Press);
        }
        "PageDown" | "PgDown" => {
            let _ = enigo.key(Key::PageDown, Press);
        }

        "Up" => {
            let _ = enigo.key(Key::UpArrow, Press);
        }
        "Down" => {
            let _ = enigo.key(Key::DownArrow, Press);
        }
        "Left" => {
            let _ = enigo.key(Key::LeftArrow, Press);
        }
        "Right" => {
            let _ = enigo.key(Key::RightArrow, Press);
        }

        "F1" => {
            let _ = enigo.key(Key::F1, Press);
        }
        "F2" => {
            let _ = enigo.key(Key::F2, Press);
        }
        "F3" => {
            let _ = enigo.key(Key::F3, Press);
        }
        "F4" => {
            let _ = enigo.key(Key::F4, Press);
        }
        "F5" => {
            let _ = enigo.key(Key::F5, Press);
        }
        "F6" => {
            let _ = enigo.key(Key::F6, Press);
        }
        "F7" => {
            let _ = enigo.key(Key::F7, Press);
        }
        "F8" => {
            let _ = enigo.key(Key::F8, Press);
        }
        "F9" => {
            let _ = enigo.key(Key::F9, Press);
        }
        "F10" => {
            let _ = enigo.key(Key::F10, Press);
        }
        "F11" => {
            let _ = enigo.key(Key::F11, Press);
        }
        "F12" => {
            let _ = enigo.key(Key::F12, Press);
        }

        _ => return Value::Error("invalid press target".into()),
    }

    Value::Bool(false)
}
