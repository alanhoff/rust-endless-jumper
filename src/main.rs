extern crate sdl2;

mod engine;
mod scenarios;
mod config;
mod helpers;

use engine::{Engine, Scene};
use scenarios::menu::Menu;

pub fn main() {
    let mut engine = Engine::new(Menu::new());
    engine.run();
}
