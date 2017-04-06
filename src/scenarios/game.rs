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

impl Game {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }
}

impl Scene for Game {
    fn on_unload(&mut self, _renderer: &mut Renderer) -> Loop {
        println!("Game unloaded");
        Loop::Continue
    }

    fn on_load(&mut self, renderer: &mut Renderer) -> Loop {
        println!("Game loaded");
        Loop::Continue
    }

    fn on_event(&mut self, event: Event, _renderer: &mut Renderer) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, renderer: &mut Renderer) -> Loop {
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        Loop::Continue
    }
}
