use cfg_if::cfg_if;

#[macro_use] extern crate log;

pub mod game;
pub mod board;
pub mod piece;
pub mod error;
pub mod ai;
pub mod r#move;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {
            env_logger::init();
        }
    }
}

pub fn init() {
    //$env:RUST_LOG="debug"
    init_log();
}