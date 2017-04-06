extern crate sdl2;

use std::collections::HashMap;

use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::EventPump;
use self::sdl2::image::{INIT_PNG, INIT_JPG};
use self::sdl2::audio::AudioCallback;

use config;

pub enum Loop {
    Continue,
    Break,
    GoToScene(String),
}

pub struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

pub struct Context {
    pub sdl2_context: &'static mut sdl2::Sdl,
    pub renderer: &'static mut Renderer<'static>,
}


pub struct Engine {
    scenarios: HashMap<String, Box<Scene>>,
    renderer: Renderer<'static>,
    event_pump: EventPump,
    sdl2_context: sdl2::Sdl,
}

impl Engine {
    pub fn add_scenario<P: Scene + 'static>(&mut self, name: String, scene: P) {
        self.scenarios.insert(name, Box::new(scene));
    }

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(&format!("{} v{}", config::TITLE, config::VERSION),
                    config::WINDOW_WIDTH,
                    config::WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Engine {
            event_pump: event_pump,
            renderer: renderer,
            scenarios: HashMap::new(),
            sdl2_context: sdl_context,
        }
    }

    pub fn run(&mut self, inital_scene: String) {
        let mut scene_name = inital_scene;

        let mut should_load = true;
        let mut should_unload = true;

        'running: loop {
            let mut scene = self.scenarios.get_mut(&scene_name).unwrap();
            let mut context = Context {
                sdl2_context: &mut self.sdl2_context,
                renderer: &mut self.renderer,
            };

            if should_load {
                should_load = false;

                match scene.on_load(&mut context) {
                    Loop::Break => break 'running,
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                        should_unload = true;
                    }
                    _ => {}
                }
            }

            for event in self.event_pump.poll_iter() {
                match scene.on_event(event, &mut context) {
                    Loop::Break => break 'running,
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                        should_unload = true;
                    }
                    _ => {}
                }
            }

            match scene.on_tick(&mut context) {
                Loop::Break => break 'running,
                Loop::GoToScene(name) => {
                    scene_name = name;
                    should_load = true;
                    should_unload = true;
                }
                _ => {}
            }


            if should_unload {
                should_unload = false;

                match scene.on_load(&mut context) {
                    Loop::Break => break 'running,
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                    }
                    _ => {}
                }
            }
        }

        let mut context = Context {
            sdl2_context: &mut self.sdl2_context,
            renderer: &mut self.renderer,
        };

        let mut scene = self.scenarios.get_mut(&scene_name).unwrap();
        scene.on_unload(&mut context);
    }
}

pub trait Scene {
    fn on_load(&mut self, &mut Context) -> Loop {
        Loop::Continue
    }
    fn on_unload(&mut self, &mut Context) -> Loop {
        Loop::Continue
    }
    fn on_event(&mut self, Event, &mut Context) -> Loop {
        Loop::Continue
    }
    fn on_tick(&mut self, &mut Context) -> Loop {
        Loop::Continue
    }
}
