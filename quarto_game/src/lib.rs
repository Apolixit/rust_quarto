use log::*;
use cfg_if::cfg_if;

#[macro_use] extern crate log;

pub mod game;
pub mod board;
pub mod piece;
pub mod error;

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
    init_log();
}