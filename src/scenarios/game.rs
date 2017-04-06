extern crate sdl2;

use std::collections::HashMap;

use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use engine::Scene;
use engine::Loop;

pub struct Game {
    textures: HashMap<String, Texture>,
}

impl Scene for Game {
    type Scenario = Self;

    fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    fn on_unload(&mut self, _renderer: &mut Renderer) {
        println!("Game unloaded");
    }

    fn on_load(&mut self, renderer: &mut Renderer) {
        println!("Game loaded");
    }

    fn on_event(&mut self, event: Event, _renderer: &mut Renderer) -> Loop<Self> {
        match event {
            Event::Quit { .. } => Loop::Break,
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, renderer: &mut Renderer) -> Loop<Self> {
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        Loop::Continue
    }
}
