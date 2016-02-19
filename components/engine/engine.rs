//use storage::{Storage};
use storage_traits::storage_thread::{Storage};
use storage_traits::environment_thread::{Environment};
use storage::storage_thread::StorageThreadFactory;
use storage::environment_thread::EnvironmentThreadFactory;
use tick_traits::tick_thread::Tick;
use tick::tick_thread::TickThreadFactory;
use ui::{Mapframe,Viewframe,WindowManager,Windowframe};

use sdl2;
use sdl2::event::{Event,WindowEventId};
use sdl2::render::Renderer;
use common::Position;
use time;

pub struct Engine {
    environment: Environment,
    storage: Storage,
    tick: Tick,
}

impl Engine {
    pub fn new() -> Self {
        let storage = Storage::new(StorageThreadFactory::new());
        let tick = Tick::new(TickThreadFactory::new());
        Engine {
            storage: storage.clone(),
            tick: tick.clone(),
            environment: Environment::new(EnvironmentThreadFactory::new(storage, tick)),
        }
    }

    pub fn run(&mut self) {
        // start sdl2 with everything
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        
        // Create a window
        let window  = match video_ctx.window("eg03", 1920, 1080).position_centered().opengl().build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

        // Create a rendering context
        let mut renderer = match window.renderer().accelerated().build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

        let mut mapframe = Mapframe::new(self.storage.clone(), self.environment.clone());

        let mut winman = WindowManager::new(Box::new(mapframe.clone()));

        mapframe.viewport.zoom = 10.0;

        winman.push_window(Windowframe::new(Box::new(mapframe)));

        let mut events = ctx.event_pump().unwrap();
        'mainloop : loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit{..} => break 'mainloop,
                    Event::Window{win_event_id, data1, data2, ..}  => {
                        match win_event_id {
                            WindowEventId::Resized | WindowEventId::SizeChanged | WindowEventId::Maximized => {
                                //self.window_size = (data1 as u32, data2 as u32);
                            }
                            _ => {}
                        }
                    }
                    Event::MouseMotion{mousestate, xrel, yrel, ..} => {
                        if mousestate.left() {
                            //self.viewport.add(-xrel, -yrel);
                        }
                    }
                    _               => {
                        println!("Got event: {:?}",  event);
                        continue
                    }
                }
            }

            renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.clear();
            winman.render(&mut renderer);
            renderer.present();

        } 
    }
}

#[test]
fn basic_test() {
    Engine::new();
}
