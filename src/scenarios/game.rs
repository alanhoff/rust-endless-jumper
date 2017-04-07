extern crate sdl2;

use std::path::Path;
use std::collections::HashMap;

use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;
use self::sdl2::image::LoadTexture;

use helpers;
use engine::{Scene, Context, Loop};

pub struct Game {
    textures: HashMap<String, Texture>,
}

impl Game {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    fn draw_background(&self, ctx: &mut Context) {
        let background_texture = self.textures.get("background".into()).unwrap();
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed());
        let size = background_texture.query();
        let mut x = ((millis % 1000) as u32 * size.width / 1000) as i32;

        for offset in 0..6 {
            let startAt = x + (-(size.width as i32));
            let destination = Rect::new(((size.width as i32 * offset as i32) + startAt) as i32,
                                        0,
                                        size.width,
                                        size.height);
            ctx.renderer
                .copy(background_texture, None, Some(destination));
        }
    }
}

impl Scene for Game {
    fn on_unload(&mut self, _ctx: &mut Context) -> Loop {
        Loop::Continue
    }

    fn on_load(&mut self, ctx: &mut Context) -> Loop {
        self.textures
            .insert("background".into(),
                    ctx.renderer
                        .load_texture(Path::new("./assets/background.png"))
                        .unwrap());
        Loop::Continue
    }

    fn on_event(&mut self, event: Event, _ctx: &mut Context) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, mut ctx: &mut Context) -> Loop {
        ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
        ctx.renderer.clear();

        self.draw_background(&mut ctx);

        ctx.renderer.present();

        Loop::Continue
    }
}
