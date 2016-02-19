//use storage::{Storage};
use storage_traits::storage_thread::{Storage};
use storage_traits::environment_thread::{Environment};
use storage::storage_thread::StorageThreadFactory;
use storage::environment_thread::EnvironmentThreadFactory;
use tick_traits::tick_thread::Tick;
use tick::tick_thread::TickThreadFactory;

use sdl2;
use sdl2::event::{Event,WindowEventId};
use sdl2::rect::{Rect};
use sdl2::render::Renderer;
use common::Position;
use time;

use viewport::Viewport;

pub struct Engine {
    viewport: Viewport,
    environment: Environment,
    storage: Storage,
    tick: Tick,
    window_size: (u32, u32),
    tile_size: u32,
    last_frame: time::Tm, 
    fps: f64,
}

impl Engine {
    pub fn new() -> Self {
        let storage = Storage::new(StorageThreadFactory::new());
        let tick = Tick::new(TickThreadFactory::new());
        Engine {
            storage: storage.clone(),
            tick: tick.clone(),
            environment: Environment::new(EnvironmentThreadFactory::new(storage, tick)),
            viewport: Viewport::default(),
            window_size: (640,480),
            tile_size: 100,
            last_frame: time::now(),
            fps: 0.0,
        }
    }

    pub fn run(&mut self) {
        // start sdl2 with everything
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        
        // Create a window
        let window  = match video_ctx.window("eg03", self.window_size.0, self.window_size.1).position_centered().opengl().build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

        // Create a rendering context
        let mut renderer = match window.renderer().build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

        let mut events = ctx.event_pump().unwrap();
        'mainloop : loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit{..} => break 'mainloop,
                    Event::Window{win_event_id, data1, data2, ..}  => {
                        match win_event_id {
                            WindowEventId::Resized | WindowEventId::SizeChanged | WindowEventId::Maximized => {
                                self.window_size = (data1 as u32, data2 as u32);
                            }
                            _ => {}
                        }
                    }
                    Event::MouseMotion{mousestate, xrel, yrel, ..} => {
                        if mousestate.left() {
                            self.viewport.add(-xrel, -yrel);
                        }
                    }
                    _               => {
                        println!("Got event: {:?}",  event);
                        continue
                    }
                }
            }

            self.render(&mut renderer);
        } 
    }

    pub fn render(&mut self, renderer: &mut Renderer){
        let (tile_size, x_tile, y_tile, x_pixels, y_pixels) = self.viewport.get_render_offsets();

        //println!("since last frame: {}", 

        self.fps = 1000000000.0 / (time::now() - self.last_frame).num_nanoseconds().unwrap() as f64;
        self.last_frame = time::now();

        //println!("fps: {}", self.fps);
        //println!("x{} y{} xpx{} ypx{}", x_tile, y_tile, x_pixels, y_pixels);

        let max_tiles = (
            (self.window_size.0 / tile_size) as i64 + 4,
            (self.window_size.1 / tile_size) as i64 + 4,
        );

        let start_tile = Position{
            x: x_tile - max_tiles.0/2 - 2,
            y: y_tile - max_tiles.1/2 - 2,
        };
        let end_tile = Position{
            x: x_tile + max_tiles.0/2,
            y: y_tile + max_tiles.1/2,
        };

        renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        renderer.clear();

        self.storage.get_area(start_tile.clone(), end_tile.clone()).and_then(|vec| {
            for (t_pos, cell) in vec {
                cell.and_then(|cell| {
                    if cell.terrain == "sand" {
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(255,255,170));
                    } else {
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(85,41,0));
                    }
                    
                    let border_rect = Rect::new(
                        (((t_pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                        (((t_pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
                        tile_size,
                        tile_size,
                    ).unwrap().unwrap();
                    //println!("drawing rect: {:?}", border_rect);
                    let _ = renderer.fill_rect(border_rect);
                    Ok(())
                });
            }
            Ok(())
        });
        
        self.environment.get_entities_by_area(start_tile.clone(), end_tile.clone()).and_then(|vec| {
            renderer.set_draw_color(sdl2::pixels::Color::RGB(238, 108, 0));
            for ent in vec {
                let border_rect = Rect::new(
                    (((ent.pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                    (((ent.pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
                    tile_size,
                    tile_size,
                ).unwrap().unwrap();
                let _ = renderer.fill_rect(border_rect);
            }
            Ok(())
        });

        renderer.present();
    }
}

#[test]
fn basic_test() {
    Engine::new();
}
