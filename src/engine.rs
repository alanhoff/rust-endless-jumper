extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::render::Renderer;
use self::sdl2::EventPump;
use self::sdl2::image::{INIT_PNG, INIT_JPG};

use config;

pub struct Engine<T: Scene> {
    scene: T,
    renderer: Renderer<'static>,
    event_pump: EventPump,
}

pub enum Loop<T: Scene> {
    Continue,
    Break,
    GoToScene(T),
}

impl<T: Scene> Engine<T>
    where <T as Scene>::Scenario: Scene
{
    pub fn new(scene: T) -> Self {
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
            scene: scene,
            event_pump: event_pump,
            renderer: renderer,
        }
    }

    pub fn run(&mut self) {
        self.scene.on_load(&mut self.renderer);

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match self.scene.on_event(event, &mut self.renderer) {
                    Loop::Break => break 'running,
                    _ => {}
                };
            }

            match self.scene.on_tick(&mut self.renderer) {
                Loop::Break => break 'running,
                _ => {}
            };
        }

        self.scene.on_unload(&mut self.renderer);
    }
}

pub trait Scene {
    type Scenario;

    fn new() -> Self::Scenario;
    fn on_load(&mut self, &mut Renderer<'static>);
    fn on_unload(&mut self, &mut Renderer<'static>);
    fn on_event(&mut self, Event, &mut Renderer<'static>) -> Loop<Self::Scenario>
        where <Self as Scene>::Scenario: Scene;
    fn on_tick(&mut self, &mut Renderer<'static>) -> Loop<Self::Scenario>
        where <Self as Scene>::Scenario: Scene;
}
