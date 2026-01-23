use crate::engine::Engine;

pub mod print;
pub mod sleep;

pub fn register_all(engine: &mut Engine) {
    print::register(engine);
    sleep::register(engine);
}