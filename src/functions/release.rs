use std::collections::HashMap;
use enigo::{
    Button,
    Direction::Release,
    Enigo, Key, Settings,
    Mouse, Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("release".into(), release);
}

fn release(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };

    match sym {
        "LMB" => { let _ = enigo.button(Button::Left, Release); }
        "MMB" => { let _ = enigo.button(Button::Middle, Release); }
        "RMB" => { let _ = enigo.button(Button::Right, Release); }
        "MB4" => { let _ = enigo.button(Button::Back, Release); }
        "MB5" => { let _ = enigo.button(Button::Forward, Release); }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Release);
        }

        "Enter" | "Return" => { let _ = enigo.key(Key::Return, Release); }
        "Space" => { let _ = enigo.key(Key::Space, Release); }
        "Tab" => { let _ = enigo.key(Key::Tab, Release); }
        "Esc" | "Escape" => { let _ = enigo.key(Key::Escape, Release); }
        "Backspace" => { let _ = enigo.key(Key::Backspace, Release); }

        "Ctrl" | "Control" => { let _ = enigo.key(Key::Control, Release); }
        "Alt" => { let _ = enigo.key(Key::Alt, Release); }
        "Shift" => { let _ = enigo.key(Key::Shift, Release); }
        "Super" | "Meta" => { let _ = enigo.key(Key::Meta, Release); }
        "CapsLock" | "Caps" => { let _ = enigo.key(Key::CapsLock, Release); }

        "Insert" => { let _ = enigo.key(Key::Insert, Release); }
        "Delete" | "Del" => { let _ = enigo.key(Key::Delete, Release); }
        "Home" => { let _ = enigo.key(Key::Home, Release); }
        "End" => { let _ = enigo.key(Key::End, Release); }
        "PageUp" | "PgUp" => { let _ = enigo.key(Key::PageUp, Release); }
        "PageDown" | "PgDown" => { let _ = enigo.key(Key::PageDown, Release); }

        "Up" => { let _ = enigo.key(Key::UpArrow, Release); }
        "Down" => { let _ = enigo.key(Key::DownArrow, Release); }
        "Left" => { let _ = enigo.key(Key::LeftArrow, Release); }
        "Right" => { let _ = enigo.key(Key::RightArrow, Release); }

        "F1" => { let _ = enigo.key(Key::F1, Release); }
        "F2" => { let _ = enigo.key(Key::F2, Release); }
        "F3" => { let _ = enigo.key(Key::F3, Release); }
        "F4" => { let _ = enigo.key(Key::F4, Release); }
        "F5" => { let _ = enigo.key(Key::F5, Release); }
        "F6" => { let _ = enigo.key(Key::F6, Release); }
        "F7" => { let _ = enigo.key(Key::F7, Release); }
        "F8" => { let _ = enigo.key(Key::F8, Release); }
        "F9" => { let _ = enigo.key(Key::F9, Release); }
        "F10" => { let _ = enigo.key(Key::F10, Release); }
        "F11" => { let _ = enigo.key(Key::F11, Release); }
        "F12" => { let _ = enigo.key(Key::F12, Release); }

        _ => {}
    }

    Value::Bool(false)
}