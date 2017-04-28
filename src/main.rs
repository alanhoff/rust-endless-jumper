extern crate sdl2;

mod engine;
mod scenarios;
mod config;
mod helpers;

use engine::{Engine, Stage};
use scenarios::menu::Menu;
use scenarios::game::Game;

pub fn main() {
    let mut stage = Stage::new();
    stage.add_scene::<Menu>("menu".into());
    stage.add_scene::<Game>("game".into());

    Engine::run("menu".into(), stage);
}
