//use backend::{Storage};
use backend_traits::storage_thread::{Storage};
use backend_traits::environment_thread::{Environment};
use backend::storage_thread::StorageThreadFactory;
use backend::environment_thread::EnvironmentThreadFactory;
use tick_traits::tick_thread::Tick;
use tick::tick_thread::TickThreadFactory;
use ui::{UI,MapBuilder,ImguiRendererEntry,TimeControls};
//use ui::{Mapframe,RootFrame,RootManager,Frame,RenderRegion,Manager,Renderer as UiRenderer,ButtonKind,ButtonMenu,Splitter,SplitterEntry,Button,PrintBroadcaster,StoredButton,WindowFactory};


pub struct Engine {
    pub environment: Environment,
    pub backend: Storage,
    pub tick: Tick,
}

impl Engine {
    pub fn new() -> Self {
        let backend = Storage::new(StorageThreadFactory::new());
        let tick = Tick::new(TickThreadFactory::new());
        Engine {
            backend: backend.clone(),
            tick: tick.clone(),
            environment: Environment::new(EnvironmentThreadFactory::new(backend, tick)),
        }
    }

    pub fn run(&mut self) {
       
        let mut ui = UI::new();
        ui.windows.push(ImguiRendererEntry{
            renderer: Box::new(TimeControls::new(self.tick.clone())),
        });
        ui.run(Box::new(MapBuilder::new(self.backend.clone(), self.environment.clone())));
        /*
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
        */
    }
}
