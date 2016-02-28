//use storage::{Storage};
use storage_traits::storage_thread::{Storage};
use storage_traits::environment_thread::{Environment};
use storage::storage_thread::StorageThreadFactory;
use storage::environment_thread::EnvironmentThreadFactory;
use tick_traits::tick_thread::Tick;
use tick::tick_thread::TickThreadFactory;
use ui::{Mapframe,RootFrame,RootManager,Frame,RenderRegion,Manager,Renderer as UiRenderer};

use sdl2;
use sdl2_ttf;
use sdl2::event::{Event};


pub struct Engine {
    pub environment: Environment,
    pub storage: Storage,
    pub tick: Tick,
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
            Err(err)   => panic!("failed to create window: {:?}", err)
        };

        // Create a rendering context
        let renderer = match window.renderer().accelerated().build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {:?}", err)
        };

        let mut rootman = RootManager::new();
        let mut mapframe = Mapframe::new(self.storage.clone(), self.environment.clone());
       
        let background_frame = mapframe.clone();

        mapframe.viewport.zoom = 10.0;
        let window = rootman.create_window(Box::new(mapframe));
        
        let mut rootframe = RootFrame::new(
            rootman.push_frame(
                RenderRegion::new(Box::new(background_frame)),
            )
        );

        rootframe.push_window(window);

        let mut renderer = UiRenderer::new(renderer, sdl2_ttf::init().unwrap());

        renderer.load_font("menu".to_string(), "assets/fonts/OpenSans-Regular.ttf".to_string(), 64);

        let mut events = ctx.event_pump().unwrap();
        'mainloop : loop {
            for event in events.poll_iter() {

                match event {
                    // todo: have the quit handled by the inner loop
                    Event::Quit{..} => break 'mainloop,
                    _ => {
                        rootframe.handle_event(&mut rootman, event);
                    }
                }
            }

            
            renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.sdl.clear();
            rootframe.begin_render(&mut rootman, &mut renderer);
            renderer.sdl.present();

        } 
    }
}

#[test]
fn basic_test() {
    Engine::new();
}
