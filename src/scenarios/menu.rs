extern crate sdl2;

use std::path::Path;
use std::collections::HashMap;

use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::Renderer;
use self::sdl2::ttf;
use self::sdl2::render::Texture;

use scenarios::game;
use helpers;
use engine::Scene;
use engine::Loop;
use config;

pub struct Menu {
    textures: HashMap<String, Texture>,
    over_play: bool,
    over_exit: bool,
}

impl Scene for Menu {
    type Scenario = Self;

    fn new() -> Self {
        Self {
            textures: HashMap::new(),
            over_play: false,
            over_exit: false,
        }
    }

    fn on_unload(&mut self, _renderer: &mut Renderer) {
        println!("Menu unloaded");
    }

    fn on_load(&mut self, renderer: &mut Renderer) {
        let ttf_context = ttf::init().unwrap();
        let mut font = ttf_context
            .load_font(Path::new("./assets/font.ttf"), 128)
            .unwrap();

        font.set_style(ttf::STYLE_BOLD);

        let title_surface = font.render("Endless Jumper")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let title_texture = renderer
            .create_texture_from_surface(&title_surface)
            .unwrap();

        self.textures.insert("title".into(), title_texture);

        let description = format!("v{} by Alan Hoffmeister", config::VERSION);
        font.set_style(ttf::STYLE_NORMAL);

        let description_surface = font.render(&description)
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let description_texture = renderer
            .create_texture_from_surface(&description_surface)
            .unwrap();

        self.textures
            .insert("description".into(), description_texture);

        let play_surface = font.render("PLAY")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let play_texture = renderer
            .create_texture_from_surface(&play_surface)
            .unwrap();

        self.textures.insert("play".into(), play_texture);

        let exit_surface = font.render("EXIT")
            .blended(Color::RGBA(0, 0, 0, 255))
            .unwrap();

        let exit_texture = renderer
            .create_texture_from_surface(&exit_surface)
            .unwrap();

        self.textures.insert("exit".into(), exit_texture);
    }

    fn on_event(&mut self, event: Event, _renderer: &mut Renderer) -> Loop<T> {
        match event {
            Event::Quit { .. } => Loop::Break,
            Event::MouseMotion { x, y, .. } => {
                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 30)) {
                    self.over_play = true;
                } else {
                    self.over_play = false;
                }

                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 100)) {
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
                    Loop::GoToScene(game::Game::new())
                } else {
                    Loop::Continue
                }
            }
            _ => Loop::Continue,
        }
    }

    fn on_tick(&mut self, renderer: &mut Renderer) -> Loop<Self> {
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();

        // Renders the title
        {
            let mut title = self.textures.get_mut("title").unwrap();
            let title_position = helpers::rect_centered(400, 50, 0, -100);

            renderer
                .copy(&mut title, None, Some(title_position))
                .unwrap();
        }

        // Renders the description
        {
            let mut description = self.textures.get_mut("description").unwrap();
            let description_position = helpers::rect_centered(200, 20, 0, -60);

            renderer
                .copy(&mut description, None, Some(description_position))
                .unwrap();
        }

        // Renders the play button
        {
            match self.over_play {
                true => renderer.set_draw_color(Color::RGB(255, 0, 0)),
                false => renderer.set_draw_color(Color::RGB(0, 0, 0)),
            }

            renderer
                .fill_rect(helpers::rect_centered(200, 60, 0, 30))
                .unwrap();

            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer
                .fill_rect(helpers::rect_centered(190, 50, 0, 30))
                .unwrap();

            let mut play = self.textures.get_mut("play").unwrap();
            let play_position = helpers::rect_centered(100, 20, 0, 30);

            renderer
                .copy(&mut play, None, Some(play_position))
                .unwrap();
        }

        // Renders the exit button
        {
            match self.over_exit {
                true => renderer.set_draw_color(Color::RGB(255, 0, 0)),
                false => renderer.set_draw_color(Color::RGB(0, 0, 0)),
            }

            renderer
                .fill_rect(helpers::rect_centered(200, 60, 0, 100))
                .unwrap();

            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer
                .fill_rect(helpers::rect_centered(190, 50, 0, 100))
                .unwrap();

            let mut exit = self.textures.get_mut("exit").unwrap();
            let exit_position = helpers::rect_centered(100, 20, 0, 100);

            renderer
                .copy(&mut exit, None, Some(exit_position))
                .unwrap();
        }

        renderer.present();
        Loop::Continue
    }
}
