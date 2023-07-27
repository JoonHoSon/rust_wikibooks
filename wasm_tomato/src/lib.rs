use wasm_bindgen::prelude::*;

mod node;
mod parser;
mod runner;

#[wasm_bindgen]
pub fn tomato_run(src: &str) -> String {
    runner::run(src)
}
