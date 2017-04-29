extern crate sdl2;

mod engine;
mod scenarios;
mod config;
mod helpers;

use engine::{Engine, Stage};
use scenarios::menu::Menu;
use scenarios::game::Game;
use scenarios::game_over::GameOver;

pub fn main() {
    let mut stage = Stage::new();
    stage.add_scene::<Menu>("menu".into());
    stage.add_scene::<Game>("game".into());
    stage.add_scene::<GameOver>("game_over".into());

    Engine::run("menu".into(), stage);
}
