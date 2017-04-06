extern crate sdl2;

use std::collections::HashMap;

use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;

use engine::{Scene, Context, Loop};

pub struct Game {
    textures: HashMap<String, Texture>,
}

impl Game {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }
}

impl Scene for Game {
    fn on_unload(&mut self, _ctx: &mut Context) -> Loop {
        println!("Game unloaded");
        Loop::Continue
    }

    fn on_load(&mut self, ctx: &mut Context) -> Loop {
        println!("Game loaded");
        Loop::Continue
    }

    fn on_event(&mut self, event: Event, _ctx: &mut Context) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, ctx: &mut Context) -> Loop {
        ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
        ctx.renderer.clear();
        ctx.renderer.present();

        Loop::Continue
    }
}
