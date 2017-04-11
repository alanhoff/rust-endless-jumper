extern crate sdl2;
extern crate rand;

use std::collections::HashMap;
use std::time::Instant;
use std::thread;

use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::EventPump;
use self::sdl2::image::{INIT_PNG, INIT_JPG};
use self::sdl2::mixer::{INIT_FLAC, AUDIO_S16LSB};
use self::rand::ThreadRng;

use helpers;
use config;

pub enum Loop {
    Continue,
    Break,
    GoToScene(String),
}

pub struct Context {
    pub sdl2_context: sdl2::Sdl,
    pub renderer: Renderer<'static>,
    pub timer: Instant,
    pub thread_rng: ThreadRng,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
}


pub struct Engine {
    scenarios: HashMap<String, Box<Scene>>,
    event_pump: EventPump,
    context: Context,
}

impl Engine {
    pub fn add_scenario<P: Scene + 'static>(&mut self, name: String, scene: P) {
        self.scenarios.insert(name, Box::new(scene));
    }

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        let window = video_subsystem
            .window(&format!("{} v{}", config::TITLE, config::VERSION),
                    config::WINDOW_WIDTH,
                    config::WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

        let renderer = window
            .renderer()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        // Setup the mixer
        sdl_context.audio().unwrap();
        sdl2::mixer::init(INIT_FLAC).unwrap();
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, 2, 1024).unwrap();
        sdl2::mixer::allocate_channels(16);

        Engine {
            event_pump: event_pump,
            scenarios: HashMap::new(),
            context: Context {
                renderer: renderer,
                sdl2_context: sdl_context,
                timer: Instant::now(),
                thread_rng: rand::thread_rng(),
                ttf_context: ttf_context,
            },
        }
    }

    pub fn run(&mut self, inital_scene: String) {
        let mut scene_name = inital_scene;

        let mut should_load = true;
        let mut should_unload = false;
        let mut timer = Instant::now();

        'running: loop {
            let elapsed = helpers::get_milliseconds(&timer.elapsed());
            if (elapsed < 15) {
                thread::sleep_ms((15 - elapsed) as u32);
            }

            timer = Instant::now();
            let mut scene = self.scenarios.get_mut(&scene_name).unwrap();

            if should_load {
                should_load = false;

                match scene.on_load(&mut self.context) {
                    Loop::Break => {
                        should_unload = true;
                        break 'running;
                    }
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                        should_unload = true;
                    }
                    _ => {}
                }
            }

            for event in self.event_pump.poll_iter() {
                match scene.on_event(event, &mut self.context) {
                    Loop::Break => {
                        should_unload = true;
                        break 'running;
                    }
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                        should_unload = true;
                    }
                    _ => {}
                }
            }

            match scene.on_tick(&mut self.context) {
                Loop::Break => {
                    should_unload = true;
                    break 'running;
                }
                Loop::GoToScene(name) => {
                    scene_name = name;
                    should_load = true;
                    should_unload = true;
                }
                _ => {}
            }


            if should_unload {
                should_unload = false;

                match scene.on_unload(&mut self.context) {
                    Loop::Break => {
                        should_unload = true;
                        break 'running;
                    }
                    Loop::GoToScene(name) => {
                        scene_name = name;
                        should_load = true;
                    }
                    _ => {}
                }
            }
        }

        let mut scene = self.scenarios.get_mut(&scene_name).unwrap();
        scene.on_unload(&mut self.context);
    }
}

pub trait Scene {
    fn on_load<'a, 'ctx: 'a>(&'a mut self, &'ctx mut Context) -> Loop {
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
