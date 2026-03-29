use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::{Context, Value};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("key".into(), key);
}

fn key(_ctx: &Context, args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("key", &args, 1) {
        return e;
    }

    let key_name = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.clone(), // Changed Value::Key to Value::Symbol
        Value::Num(n) => n.to_string(),
        _ => return Value::Error("key expects a key name (string or symbol)".into()),
    };

    let device = DeviceState::new();
    let keys = device.get_keys();
    
    let pressed = match key_name.as_str() {
        "Ctrl" | "Control" => keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl),
        "LCtrl" | "LControl" => keys.contains(&Keycode::LControl),
        "RCtrl" | "RControl" => keys.contains(&Keycode::RControl),
        "Shift" => keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift),
        "LShift" => keys.contains(&Keycode::LShift),
        "RShift" => keys.contains(&Keycode::RShift),
        "Alt" => keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt),
        "LAlt" => keys.contains(&Keycode::LAlt),
        "RAlt" => keys.contains(&Keycode::RAlt),
        "Super" | "Meta" => keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta),
        "Enter" | "Return" => keys.contains(&Keycode::Enter),
        "Space" => keys.contains(&Keycode::Space),
        "Tab" => keys.contains(&Keycode::Tab),
        "Esc" | "Escape" => keys.contains(&Keycode::Escape),
        "Backspace" => keys.contains(&Keycode::Backspace),
        "Up" => keys.contains(&Keycode::Up),
        "Down" => keys.contains(&Keycode::Down),
        "Left" => keys.contains(&Keycode::Left),
        "Right" => keys.contains(&Keycode::Right),
        "F1" => keys.contains(&Keycode::F1),
        "F2" => keys.contains(&Keycode::F2),
        "F3" => keys.contains(&Keycode::F3),
        "F4" => keys.contains(&Keycode::F4),
        "F5" => keys.contains(&Keycode::F5),
        "F6" => keys.contains(&Keycode::F6),
        "F7" => keys.contains(&Keycode::F7),
        "F8" => keys.contains(&Keycode::F8),
        "F9" => keys.contains(&Keycode::F9),
        "F10" => keys.contains(&Keycode::F10),
        "F11" => keys.contains(&Keycode::F11),
        "F12" => keys.contains(&Keycode::F12),
        k if k.len() == 1 => {
            let c = k.chars().next().unwrap().to_ascii_lowercase();
            keys.iter().any(|kc| match kc {
                Keycode::A => c == 'a', Keycode::B => c == 'b', Keycode::C => c == 'c',
                Keycode::D => c == 'd', Keycode::E => c == 'e', Keycode::F => c == 'f',
                Keycode::G => c == 'g', Keycode::H => c == 'h', Keycode::I => c == 'i',
                Keycode::J => c == 'j', Keycode::K => c == 'k', Keycode::L => c == 'l',
                Keycode::M => c == 'm', Keycode::N => c == 'n', Keycode::O => c == 'o',
                Keycode::P => c == 'p', Keycode::Q => c == 'q', Keycode::R => c == 'r',
                Keycode::S => c == 's', Keycode::T => c == 't', Keycode::U => c == 'u',
                Keycode::V => c == 'v', Keycode::W => c == 'w', Keycode::X => c == 'x',
                Keycode::Y => c == 'y', Keycode::Z => c == 'z', 
                Keycode::Key0 => c == '0', Keycode::Key1 => c == '1', Keycode::Key2 => c == '2',
                Keycode::Key3 => c == '3', Keycode::Key4 => c == '4', Keycode::Key5 => c == '5',
                Keycode::Key6 => c == '6', Keycode::Key7 => c == '7', Keycode::Key8 => c == '8',
                Keycode::Key9 => c == '9',
                _ => false,
            })
        }
        _ => false,
    };

    Value::Bool(pressed)
}
