use std::collections::HashMap;
use enigo::{
    Button,
    Direction::Click,
    Enigo, Key, Settings,
    Mouse, Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("click".into(), click);
}

fn click(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };

    match sym {
        "LMB" => { let _ = enigo.button(Button::Left, Click); }
        "MMB" => { let _ = enigo.button(Button::Middle, Click); }
        "RMB" => { let _ = enigo.button(Button::Right, Click); }
        "MB4" => { let _ = enigo.button(Button::Back, Click); }
        "MB5" => { let _ = enigo.button(Button::Forward, Click); }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Click);
        }

        "Enter" | "Return" => { let _ = enigo.key(Key::Return, Click); }
        "Space" => { let _ = enigo.key(Key::Space, Click); }
        "Tab" => { let _ = enigo.key(Key::Tab, Click); }
        "Esc" | "Escape" => { let _ = enigo.key(Key::Escape, Click); }
        "Backspace" => { let _ = enigo.key(Key::Backspace, Click); }

        "Ctrl" | "Control" => { let _ = enigo.key(Key::Control, Click); }
        "Alt" => { let _ = enigo.key(Key::Alt, Click); }
        "Shift" => { let _ = enigo.key(Key::Shift, Click); }
        "Super" | "Meta" => { let _ = enigo.key(Key::Meta, Click); }
        "CapsLock" | "Caps" => { let _ = enigo.key(Key::CapsLock, Click); }

        "Insert" => { let _ = enigo.key(Key::Insert, Click); }
        "Delete" | "Del" => { let _ = enigo.key(Key::Delete, Click); }
        "Home" => { let _ = enigo.key(Key::Home, Click); }
        "End" => { let _ = enigo.key(Key::End, Click); }
        "PageUp" | "PgUp" => { let _ = enigo.key(Key::PageUp, Click); }
        "PageDown" | "PgDown" => { let _ = enigo.key(Key::PageDown, Click); }

        "Up" => { let _ = enigo.key(Key::UpArrow, Click); }
        "Down" => { let _ = enigo.key(Key::DownArrow, Click); }
        "Left" => { let _ = enigo.key(Key::LeftArrow, Click); }
        "Right" => { let _ = enigo.key(Key::RightArrow, Click); }

        "F1" => { let _ = enigo.key(Key::F1, Click); }
        "F2" => { let _ = enigo.key(Key::F2, Click); }
        "F3" => { let _ = enigo.key(Key::F3, Click); }
        "F4" => { let _ = enigo.key(Key::F4, Click); }
        "F5" => { let _ = enigo.key(Key::F5, Click); }
        "F6" => { let _ = enigo.key(Key::F6, Click); }
        "F7" => { let _ = enigo.key(Key::F7, Click); }
        "F8" => { let _ = enigo.key(Key::F8, Click); }
        "F9" => { let _ = enigo.key(Key::F9, Click); }
        "F10" => { let _ = enigo.key(Key::F10, Click); }
        "F11" => { let _ = enigo.key(Key::F11, Click); }
        "F12" => { let _ = enigo.key(Key::F12, Click); }

        _ => {}
    }

    Value::Bool(false)
}