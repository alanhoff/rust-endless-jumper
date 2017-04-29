extern crate sdl2;

use std::path::Path;
use std::collections::HashMap;

use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::ttf;
use self::sdl2::render::Texture;
use self::sdl2::mixer::channel;

use helpers;
use engine::{Scene, Loop, Context};

pub struct GameOver {
    textures: HashMap<String, Texture>,
    over_exit: bool,
}

impl<'a> Scene for GameOver {
    fn new() -> Self {
        Self { textures: HashMap::new(), over_exit: false }
    }

    fn on_event(&mut self, event: Event, ctx: &mut Context) -> Loop {
        match event {
            Event::Quit { .. } => Loop::Break,
            Event::MouseMotion { x, y, .. } => {
                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 100)) {
                    if !self.over_exit {
                        channel(1)
                            .play(ctx.sounds.get("menu").unwrap(), 0)
                            .unwrap();
                    }

                    self.over_exit = true;
                } else {
                    self.over_exit = false;
                }
                Loop::Continue
            },
            Event::MouseButtonUp { x, y, .. } => {
                if helpers::point_colliding_rect(x, y, &helpers::rect_centered(200, 60, 0, 100)) {
                    Loop::Break
                } else {
                    Loop::Continue
                }
            },
            _ => Loop::Continue,
        }
    }

    fn on_load(&mut self, ctx: &mut Context) -> Loop {
        let mut font = ctx.ttf_context
            .load_font(Path::new("./assets/font.ttf"), 128)
            .unwrap();


        font.set_style(ttf::STYLE_BOLD);

        let title_surface = font.render("Game Over")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        let title_texture = ctx.renderer
            .create_texture_from_surface(&title_surface)
            .unwrap();

        self.textures.insert("game_over".into(), title_texture);

        let subtitle_surface =
            font.render("You ran reckless through the woods and now you're dead..")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        self.textures
            .insert("subtitle".into(),
                    ctx.renderer
                        .create_texture_from_surface(&subtitle_surface)
                        .unwrap());

        let points_text_surface =
            font.render("Don't be sad, at least you made some points:")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        self.textures
            .insert("points_text".into(),
                    ctx.renderer
                        .create_texture_from_surface(&points_text_surface)
                        .unwrap());

        let points_surface =
            font.render(&ctx.registry.get("points".into()).unwrap().to_string())
            .blended(Color::RGBA(0, 0, 255, 255))
            .unwrap();

        self.textures
            .insert("points".into(),
                    ctx.renderer
                        .create_texture_from_surface(&points_surface)
                        .unwrap());

        let exit_surface =
            font.render("EXIT")
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();

        self.textures
            .insert("exit".into(),
                    ctx.renderer
                        .create_texture_from_surface(&exit_surface)
                        .unwrap());

        Loop::Continue
    }

    fn on_tick(&mut self, ctx: &mut Context) -> Loop {
        ctx.renderer.set_draw_color(Color::RGB(0, 0, 0));
        ctx.renderer.clear();

        // Renders the title
        {
            let mut game_over = self.textures.get_mut("game_over").unwrap();
            let position = helpers::rect_centered(400, 50, 0, -200);

            ctx.renderer
                .copy(&mut game_over, None, Some(position))
                .unwrap();
        }

        // Renders the subtitle
        {
            let mut subtitle = self.textures.get_mut("subtitle").unwrap();
            let position = helpers::rect_centered(560, 30, 0, -140);

            ctx.renderer
                .copy(&mut subtitle, None, Some(position))
                .unwrap();
        }

        // Renders the points_text
        {
            let mut text = self.textures.get_mut("points_text").unwrap();
            let position = helpers::rect_centered(430, 30, 0, -100);

            ctx.renderer
                .copy(&mut text, None, Some(position))
                .unwrap();
        }

        // Renders points
        {
            let mut text = self.textures.get_mut("points").unwrap();
            let points = ctx.registry.get("points".into()).unwrap().to_string();
            let width = points.len() * 33;
            let position = helpers::rect_centered(width as i32, 60, 0, 0);

            ctx.renderer
                .copy(&mut text, None, Some(position))
                .unwrap();
        }

        // Renders the exit button
        {
            match self.over_exit {
                true => ctx.renderer.set_draw_color(Color::RGB(255, 0, 0)),
                false => ctx.renderer.set_draw_color(Color::RGB(255, 255, 255)),
            }

            ctx.renderer
                .fill_rect(helpers::rect_centered(200, 60, 0, 100))
                .unwrap();

            ctx.renderer.set_draw_color(Color::RGB(0, 0, 0));
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
