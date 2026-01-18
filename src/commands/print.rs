use crate::engine::{Engine, Value};

pub fn register(engine: &mut Engine) {
    engine.add_command("print", |args, _| {
        for a in args {
            print!("{}", a);
        }
        println!();
        Ok(())
    });
}