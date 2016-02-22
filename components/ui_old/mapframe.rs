use super::*;
use common::Position;
use storage_traits::storage_thread::Storage;
use storage_traits::environment_thread::Environment;

use sdl2::surface::Surface;
use sdl2::render::Renderer;
use sdl2::rect::{Rect};
use sdl2::event::Event;

#[derive(Clone)]
pub struct Mapframe {
    pub viewport: Viewport,
    fps: f64,
    last_frame: time::Tm,
    storage: Storage,
    environment: Environment,
}

impl Mapframe {
    pub fn new(storage: Storage, environment: Environment) -> Self {
        Mapframe{
            viewport: Viewport::default(),
            fps: 0.0,
            last_frame: time::now(),
            storage: storage,
            environment: environment,
        }
    }
}

impl Viewframe for Mapframe {
    fn render(&mut self, renderer: &mut Renderer) {
        let window_size = renderer.viewport();
        //println!("window size: {:?}", window_size);

        let (tile_size, x_tile, y_tile, x_pixels, y_pixels) = self.viewport.get_render_offsets();

        //println!("since last frame: {}", 

        self.fps = 1000000000.0 / (time::now() - self.last_frame).num_nanoseconds().unwrap() as f64;
        self.last_frame = time::now();

        //println!("fps: {}", self.fps);
        //println!("x{} y{} xpx{} ypx{}", x_tile, y_tile, x_pixels, y_pixels);

        let max_tiles = (
            (window_size.width() / tile_size) as i64 + 4,
            (window_size.height() / tile_size) as i64 + 4,
        );

        let start_tile = Position{
            x: x_tile - max_tiles.0/2,
            y: y_tile - max_tiles.1/2,
        };
        let end_tile = Position{
            x: x_tile + max_tiles.0/2,
            y: y_tile + max_tiles.1/2,
        };

        let recenter = (
            -(tile_size as i32 /2 as i32) + (((x_tile - start_tile.x) * tile_size as i64) - (window_size.width() / 2) as i64) as i32,
            -(tile_size as i32/2 as i32) + (((y_tile - start_tile.y) * tile_size as i64) - (window_size.height() / 2) as i64) as i32,
        );

        self.storage.get_area(start_tile.clone(), end_tile.clone()).and_then(|vec| {
            for (t_pos, cell) in vec {
                cell.and_then(|cell| {
                    if cell.terrain == "sand" {
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(255,255,170));
                    } else {
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(85,41,0));
                    }
                    
                    let border_rect = Rect::new(
                        -recenter.0 + (((t_pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                        -recenter.1 + (((t_pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
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
                    -recenter.0 + (((ent.pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                    -recenter.1 +(((ent.pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
                    tile_size,
                    tile_size,
                ).unwrap().unwrap();
                let _ = renderer.fill_rect(border_rect);
            }
            Ok(())
        });

    }

    fn handle_event(&mut self, ev: Event) {
        println!("MAPFRAME Got event: {:?}",  ev);
    }
}
