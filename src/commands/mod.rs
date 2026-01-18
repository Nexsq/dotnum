use crate::engine::Engine;

mod print;
mod sleep;

pub fn register_all(engine: &mut Engine) {
    print::register(engine);
    sleep::register(engine);
}