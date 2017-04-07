extern crate sdl2;

use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;

use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::render::Texture;
use self::sdl2::image::LoadTexture;
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer::{Chunk, channel};

use helpers;
use engine::{Scene, Context, Loop};

const MONTAINS_SCALE: u32 = 3;
const GROUND_SCALE: u32 = 2;
const PLAYER_SCALE: i32 = 3;
const GRAVITY: f32 = 0.5;
const ROCK_SCALE: i32 = 2;

pub struct Game {
    textures: HashMap<String, Texture>,
    timers: HashMap<String, Instant>,
    velocity_y: f32,
    position_y: f32,
    jumping: bool,
    released: bool,
    jump: Chunk,
}

impl Game {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            timers: HashMap::new(),
            velocity_y: 0.00,
            position_y: 400.00,
            jumping: false,
            released: true,
            jump: Chunk::from_file(Path::new("./assets/jump.wav")).unwrap(),
        }
    }

    fn physics(&mut self, ctx: &mut Context) {
        self.velocity_y += GRAVITY;
        self.position_y += self.velocity_y;

        if (self.position_y > 400.00) {
            self.jumping = false;
            self.position_y = 400.00;
            self.velocity_y = 0.00;
        }
    }

    fn draw_montains(&self, ctx: &mut Context) {
        let montains_texture = self.textures.get("montains".into()).unwrap();
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed());
        let window_width = (ctx.renderer.window().unwrap().size().0) as i32;
        let window_height = (ctx.renderer.window().unwrap().size().1) as i32;
        let image_width = (montains_texture.query().width * MONTAINS_SCALE) as i32;
        let image_height = (montains_texture.query().height * MONTAINS_SCALE) as i32;
        let x = ((millis % 15000) as i32 * image_width / 15000) as i32;

        for n in 0..2 {
            let offset = n as i32;
            let startAt = x + (-(image_width));
            let destination = Rect::new((image_width * offset + startAt) as i32,
                                        window_height - image_height,
                                        image_width as u32,
                                        image_height as u32);
            ctx.renderer
                .copy(montains_texture, None, Some(destination))
                .unwrap();
        }
    }

    fn draw_player(&self, ctx: &mut Context) {
        let player = self.textures.get("player".into()).unwrap();
        let sprite_row = 3i32;
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed());
        let sprite_column = ((millis % 800) / 100) as i32;
        let image_width = 46i32;
        let image_height = 50i32;

        let window_width = (ctx.renderer.window().unwrap().size().0) as i32;
        let sprite = Rect::new(sprite_column * image_width,
                               sprite_row * image_height,
                               image_width as u32,
                               image_height as u32);


        let destination = Rect::new(500,
                                    self.position_y as i32,
                                    (image_width * PLAYER_SCALE) as u32,
                                    (image_height * PLAYER_SCALE) as u32);
        ctx.renderer
            .copy_ex(&player,
                     Some(sprite),
                     Some(destination),
                     0.00,
                     None,
                     true,
                     false)
            .unwrap();
    }

    fn draw_rock(&self, ctx: &mut Context) {
        let rock = self.textures.get("rocks".into()).unwrap();
        let sprite_row = 0i32;
        let sprite_column = 3i32;
        let image_width = 32i32;
        let image_height = 32i32;
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed()) as i32;
        let x = ((millis % 1560) as i32 * (800 + image_width) / 1560) as i32;
        let sprite = Rect::new(sprite_column * image_width,
                               sprite_row * image_height,
                               image_width as u32,
                               image_height as u32);


        for n in 0..6 {
            let destination = Rect::new(x - image_width,
                                        480i32 - (n * (image_height - 15)),
                                        (image_width * ROCK_SCALE) as u32,
                                        (image_height * ROCK_SCALE) as u32);
            ctx.renderer
                .copy_ex(&rock,
                         Some(sprite),
                         Some(destination),
                         0.00,
                         None,
                         true,
                         false)
                .unwrap();
        }
    }

    fn draw_jump(&self, ctx: &mut Context) {
        let player = self.textures.get("player".into()).unwrap();
        let sprite_row = 0i32;
        let sprite_column = {
            if (self.velocity_y < 0.00) { 6 } else { 7 }
        } as i32;
        let image_width = 46i32;
        let image_height = 50i32;

        let window_width = (ctx.renderer.window().unwrap().size().0) as i32;
        let sprite = Rect::new(sprite_column * image_width,
                               sprite_row * image_height,
                               image_width as u32,
                               image_height as u32);


        let destination = Rect::new(500,
                                    self.position_y as i32,
                                    (image_width * PLAYER_SCALE) as u32,
                                    (image_height * PLAYER_SCALE) as u32);
        ctx.renderer
            .copy_ex(&player,
                     Some(sprite),
                     Some(destination),
                     0.00,
                     None,
                     true,
                     false)
            .unwrap();
    }

    fn draw_forest(&self, ctx: &mut Context) {
        let forest_texture = self.textures.get("forest".into()).unwrap();
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed());
        let window_width = (ctx.renderer.window().unwrap().size().0) as i32;
        let window_height = (ctx.renderer.window().unwrap().size().1) as i32;
        let image_width = (forest_texture.query().width * MONTAINS_SCALE) as i32;
        let image_height = (forest_texture.query().height * MONTAINS_SCALE) as i32;
        let x = ((millis % 5000) as i32 * image_width / 5000) as i32;

        for n in 0..3 {
            let offset = n as i32;
            let startAt = x + (-(image_width));
            let destination = Rect::new((image_width * offset + startAt) as i32,
                                        window_height - image_height,
                                        image_width as u32,
                                        image_height as u32);
            ctx.renderer
                .copy(forest_texture, None, Some(destination))
                .unwrap();
        }
    }

    fn draw_ground(&self, ctx: &mut Context) {
        let background_texture = self.textures.get("background".into()).unwrap();
        let millis = helpers::get_milliseconds(&ctx.timer.elapsed());
        let window_width = (ctx.renderer.window().unwrap().size().0) as i32;
        let window_height = (ctx.renderer.window().unwrap().size().1) as i32;
        let image_width = (background_texture.query().width * GROUND_SCALE) as i32;
        let image_height = (background_texture.query().height * GROUND_SCALE) as i32;
        let x = ((millis % 1500) as i32 * image_width / 1500) as i32;

        for n in 0..3 {
            let offset = n as i32;
            let startAt = x + (-(image_width));
            let destination = Rect::new((image_width * offset + startAt) as i32,
                                        window_height - image_height,
                                        image_width as u32,
                                        image_height as u32);
            ctx.renderer
                .copy(background_texture, None, Some(destination))
                .unwrap();
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

        self.textures
            .insert("montains".into(),
                    ctx.renderer
                        .load_texture(Path::new("./assets/background-montains.png"))
                        .unwrap());

        self.textures
            .insert("forest".into(),
                    ctx.renderer
                        .load_texture(Path::new("./assets/background-forest.png"))
                        .unwrap());

        self.textures
            .insert("player".into(),
                    ctx.renderer
                        .load_texture(Path::new("./assets/player.png"))
                        .unwrap());

        self.textures
            .insert("rocks".into(),
                    ctx.renderer
                        .load_texture(Path::new("./assets/rocks.png"))
                        .unwrap());
        Loop::Continue
    }

    fn on_event(&mut self, event: Event, _ctx: &mut Context) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                if !self.jumping && self.released {
                    self.released = false;
                    self.jumping = true;
                    self.velocity_y = -16.00;
                    channel(1).play(&self.jump, 0).unwrap();
                }
                Loop::Continue
            }
            Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                self.released = true;

                if self.jumping && self.velocity_y < -6.00 {
                    self.velocity_y = -6.00;
                }
                Loop::Continue
            }
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, mut ctx: &mut Context) -> Loop {
        ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
        ctx.renderer.clear();


        self.physics(&mut ctx);
        self.draw_montains(&mut ctx);
        self.draw_forest(&mut ctx);
        self.draw_ground(&mut ctx);
        self.draw_rock(&mut ctx);

        if (self.jumping) {
            self.draw_jump(&mut ctx);
        } else {
            self.draw_player(&mut ctx);
        }

        ctx.renderer.present();

        Loop::Continue
    }
}
