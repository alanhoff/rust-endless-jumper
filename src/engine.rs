extern crate sdl2;

use std::collections::HashMap;

use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::EventPump;
use self::sdl2::image::{INIT_PNG, INIT_JPG};

use config;

pub enum Loop {
    Continue,
    Break,
    GoToScene(String),
}

pub struct Engine {
    scenarios: HashMap<String, Box<Scene>>,
    renderer: Renderer<'static>,
    event_pump: EventPump,
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
        }
    }

    pub fn run(&mut self, inital_scene: String) {
        let mut scene_name = inital_scene;

        let mut should_load = true;
        let mut should_unload = true;

        'running: loop {
            let mut scene = self.scenarios.get_mut(&scene_name).unwrap();

            if should_load {
                should_load = false;

                match scene.on_load(&mut self.renderer) {
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
                match scene.on_event(event, &mut self.renderer) {
                    Loop::Break => break 'running,
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                        should_unload = true;
                    }
                    _ => {}
                }
            }

            match scene.on_tick(&mut self.renderer) {
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

                match scene.on_load(&mut self.renderer) {
                    Loop::Break => break 'running,
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                    }
                    _ => {}
                }
            }
        }

        let mut scene = self.scenarios.get_mut(&scene_name).unwrap();
        scene.on_unload(&mut self.renderer);
    }
}

pub trait Scene {
    fn on_load(&mut self, &mut Renderer<'static>) -> Loop {
        Loop::Continue
    }
    fn on_unload(&mut self, &mut Renderer<'static>) -> Loop {
        Loop::Continue
    }
    fn on_event(&mut self, Event, &mut Renderer<'static>) -> Loop {
        Loop::Continue
    }
    fn on_tick(&mut self, &mut Renderer<'static>) -> Loop {
        Loop::Continue
    }
}
