#![allow(unused_assignments)]
extern crate sdl2;
extern crate rand;

use std::fmt;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::thread;

use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::image::{INIT_PNG, INIT_JPG};
use self::sdl2::mixer::{INIT_FLAC, AUDIO_S16LSB, Chunk};
use self::rand::ThreadRng;
use self::sdl2::ttf::{Font, Sdl2TtfContext};

use helpers;
use config;

pub enum Loop {
    Continue,
    Break,
    GoToScene(String),
}

pub enum RegistryItem {
    Number(usize),
}

impl fmt::Display for RegistryItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &RegistryItem::Number(ref number) => number,
               })
    }
}

pub struct Context<'a> {
    pub sdl2_context: sdl2::Sdl,
    pub renderer: Renderer<'a>,
    pub timer: Instant,
    pub thread_rng: ThreadRng,
    pub ttf_context: &'a Sdl2TtfContext,
    pub fonts: HashMap<String, Font<'a, 'static>>,
    pub sounds: HashMap<String, Chunk>,
    pub registry: HashMap<String, RegistryItem>,
}


pub struct Engine {}

pub struct Stage<'a> {
    pub scenarios: HashMap<String, Box<Scene + 'a>>,
}

impl<'a> Stage<'a> {
    pub fn new() -> Self {
        Self { scenarios: HashMap::new() }
    }

    pub fn add_scene<S: Scene + 'a>(&mut self, name: String) {
        self.scenarios.insert(name, Box::new(S::new()));
    }
}

impl<'a> Engine {
    pub fn run(inital_scene: String, mut stage: Stage) {
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

        let mut event_pump = sdl_context.event_pump().unwrap();

        // Setup the mixer
        sdl_context.audio().unwrap();
        sdl2::mixer::init(INIT_FLAC).unwrap();
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, 2, 1024).unwrap();
        sdl2::mixer::allocate_channels(16);

        let mut context = Context {
            renderer: renderer,
            sdl2_context: sdl_context,
            timer: Instant::now(),
            thread_rng: rand::thread_rng(),
            ttf_context: &ttf_context,
            fonts: HashMap::new(),
            sounds: HashMap::new(),
            registry: HashMap::new(),
        };

        {
            let mut scene_name = inital_scene;
            let mut should_load = true;
            let mut should_unload = false;
            let mut timer = Instant::now();

            'running: loop {
                let elapsed = helpers::get_milliseconds(&timer.elapsed());
                if elapsed < 15 {
                    thread::sleep(Duration::from_millis(15 - elapsed));
                }

                timer = Instant::now();
                let ref mut scene = stage.scenarios.get_mut(&scene_name).unwrap();

                if should_load {
                    should_load = false;
                    match scene.on_load(&mut context) {
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

                for event in event_pump.poll_iter() {
                    match scene.on_event(event, &mut context) {
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

                match scene.on_tick(&mut context) {
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

                    match scene.on_unload(&mut context) {
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

            let ref mut scene = stage.scenarios.get_mut(&scene_name).unwrap();
            scene.on_unload(&mut context);
        }
    }
}

pub trait Scene {
    fn new() -> Self where Self: Sized;

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
