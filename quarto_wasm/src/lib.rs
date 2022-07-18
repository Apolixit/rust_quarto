use std::panic;
use cfg_if::cfg_if;
use log::info;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    init_log();
    info!("Rust logs initialized");

    //Panic display in the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// #[wasm_bindgen]
// pub fn start_player_vs_player(p1: String, p2: String) -> Result<Game, JsError> {
//     Ok(Game::start(p1, p2))
// }

// #[wasm_bindgen]
// pub fn start_player_vs_ai(p1: String) -> Result<Game, JsError> {

// }