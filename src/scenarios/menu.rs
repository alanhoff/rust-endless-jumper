extern crate sdl2;

use std::path::Path;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::Renderer;
use self::sdl2::ttf;
use self::sdl2::render::Texture;
use self::sdl2::mixer::{Music, Chunk, channel};

use scenarios::game;
use helpers;
use engine::{Scene, Loop, Context};
use config;

pub struct Menu {
    textures: HashMap<String, Texture>,
    over_play: bool,
    over_exit: bool,
    music: Chunk,
    menu: Chunk,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            over_play: false,
            over_exit: false,
            music: Chunk::from_file(Path::new("./assets/music.wav")).unwrap(),
            menu: Chunk::from_file(Path::new("./assets/menu.wav")).unwrap(),
        }
    }
}

impl<'a> Scene<'a> for Menu {
    fn on_load(&mut self, mut ctx: &'a mut Context) -> Loop {
        channel(0).play(&self.music, -1).unwrap();

        let ttf_context = ttf::init().unwrap();
        let mut font = ttf_context
            .load_font(Path::new("./assets/font.ttf"), 128)
            .unwrap();


        font.set_style(ttf::STYLE_BOLD);

        let title_surface = font.render("Endless Jumper")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let title_texture = ctx.renderer
            .create_texture_from_surface(&title_surface)
            .unwrap();

        self.textures.insert("title".into(), title_texture);

        let description = format!("v{} by Alan Hoffmeister", config::VERSION);
        font.set_style(ttf::STYLE_NORMAL);

        let description_surface = font.render(&description)
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let description_texture = ctx.renderer
            .create_texture_from_surface(&description_surface)
            .unwrap();

        self.textures
            .insert("description".into(), description_texture);

        let play_surface = font.render("PLAY")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let play_texture = ctx.renderer
            .create_texture_from_surface(&play_surface)
            .unwrap();

        self.textures.insert("play".into(), play_texture);

        let exit_surface = font.render("EXIT")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let exit_texture = ctx.renderer
            .create_texture_from_surface(&exit_surface)
            .unwrap();

        self.textures.insert("exit".into(), exit_texture);

        Loop::Continue
    }

    fn on_event(&mut self, event: Event, _ctx: &mut Context) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            Event::MouseMotion { x, y, .. } => {
                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 30)) {
                    if !self.over_play {
                        channel(1).play(&self.menu, 0).unwrap();
                    }

                    self.over_play = true;
                } else {
                    self.over_play = false;
                }

                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 100)) {
                    if !self.over_exit {
                        channel(1).play(&self.menu, 0).unwrap();
                    }

                    self.over_exit = true;
                } else {
                    self.over_exit = false;
                }

                Loop::Continue
            }
            Event::MouseButtonUp { x, y, .. } => {
                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 100)) {
                    Loop::Break
                } else if helpers::point_colliding_rect(x,
                                                        y,
                                                        &helpers::rect_centered(200, 60, 0, 30)) {

                    Loop::GoToScene("game".into())
                } else {
                    Loop::Continue
                }
            }
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, ctx: &mut Context) -> Loop {
        ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
        ctx.renderer.clear();

        // Renders the title
        {
            let mut title = self.textures.get_mut("title").unwrap();
            let title_position = helpers::rect_centered(400, 50, 0, -100);

            ctx.renderer
                .copy(&mut title, None, Some(title_position))
                .unwrap();
        }

        // Renders the description
        {
            let mut description = self.textures.get_mut("description").unwrap();
            let description_position = helpers::rect_centered(200, 20, 0, -60);

            ctx.renderer
                .copy(&mut description, None, Some(description_position))
                .unwrap();
        }

        // Renders the play button
        {
            match self.over_play {
                true => ctx.renderer.set_draw_color(Color::RGB(255, 0, 0)),
                false => ctx.renderer.set_draw_color(Color::RGB(0, 0, 0)),
            }

            ctx.renderer
                .fill_rect(helpers::rect_centered(200, 60, 0, 30))
                .unwrap();

            ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
            ctx.renderer
                .fill_rect(helpers::rect_centered(190, 50, 0, 30))
                .unwrap();

            let mut play = self.textures.get_mut("play").unwrap();
            let play_position = helpers::rect_centered(100, 20, 0, 30);

            ctx.renderer
                .copy(&mut play, None, Some(play_position))
                .unwrap();
        }

        // Renders the exit button
        {
            match self.over_exit {
                true => ctx.renderer.set_draw_color(Color::RGB(255, 0, 0)),
                false => ctx.renderer.set_draw_color(Color::RGB(0, 0, 0)),
            }

            ctx.renderer
                .fill_rect(helpers::rect_centered(200, 60, 0, 100))
                .unwrap();

            ctx.renderer.set_draw_color(Color::RGB(255, 255, 255));
            ctx.renderer
                .fill_rect(helpers::rect_centered(190, 50, 0, 100))
                .unwrap();

            let mut exit = self.textures.get_mut("exit").unwrap();
            let exit_position = helpers::rect_centered(100, 20, 0, 100);

            ctx.renderer
                .copy(&mut exit, None, Some(exit_position))
                .unwrap();
        }

        ctx.renderer.present();
        Loop::Continue
    }
}
