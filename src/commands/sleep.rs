use crate::engine::{Engine, Value};
use std::{thread, time::Duration};

pub fn register(engine: &mut Engine) {
    engine.add_command("sleep", |args, _| {
        let ms = args[0].as_i64()? as u64;
        thread::sleep(Duration::from_millis(ms));
        Ok(())
    });
}