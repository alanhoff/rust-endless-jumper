extern crate sdl2;

mod engine;
mod scenarios;
mod config;
mod helpers;

use engine::{Engine, Scene};
use scenarios::menu::Menu;
use scenarios::game::Game;

pub fn main() {
    let mut engine = Engine::new();
    engine.add_scenario("menu".into(), Menu::new());
    engine.add_scenario("game".into(), Game::new());
    engine.run("menu".into());
}
