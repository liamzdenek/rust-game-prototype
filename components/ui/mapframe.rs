use super::*;
use common::Position;
use storage_traits::storage_thread::Storage;
use storage_traits::environment_thread::Environment;

//use sdl2::surface::Surface;
//use sdl2::render::Renderer;
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

impl Frame for Mapframe {
    fn render(&mut self, manager: &mut Manager, window_size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        let _ = manager; // make unused var shut up
        //println!("map frame rendering at: {:?}", window_size);
        let (tile_size, x_tile, y_tile, x_pixels, y_pixels) = self.viewport.get_render_offsets();

        //println!("since last frame: {}", 

        self.fps = 1000000000.0 / (time::now() - self.last_frame).num_nanoseconds().unwrap() as f64;
        self.last_frame = time::now();

        //println!("fps: {}", self.fps);
        //println!("x{} y{} xpx{} ypx{}", x_tile, y_tile, x_pixels, y_pixels);

        let max_tiles = (
            (window_size.w / tile_size) as i64 + 4,
            (window_size.h / tile_size) as i64 + 4,
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
            -window_size.x + -(tile_size as i32 /2 as i32) + (((x_tile - start_tile.x) * tile_size as i64) - (window_size.w / 2) as i64) as i32,
            -window_size.y + -(tile_size as i32/2 as i32) + (((y_tile - start_tile.y) * tile_size as i64) - (window_size.h / 2) as i64) as i32,
        );

        let _ = self.storage.get_area(start_tile.clone(), end_tile.clone()).and_then(|vec| {
            for (t_pos, cell) in vec {
                let _ = cell.and_then(|cell| {
                    if cell.terrain == "sand" {
                        renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(255,255,170));
                    } else {
                        renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(85,41,0));
                    }
                    
                    let border_rect = Rect::new(
                        -recenter.0 + (((t_pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                        -recenter.1 + (((t_pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
                        tile_size,
                        tile_size,
                    );
                    //println!("drawing rect: {:?}", border_rect);
                    let _ = renderer.sdl.fill_rect(border_rect);
                    Ok(())
                });
            }
            Ok(())
        });
        
        let _ = self.environment.get_entities_by_area(start_tile.clone(), end_tile.clone()).and_then(|vec| {
            renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(238, 108, 0));
            for ent in vec {
                let border_rect = Rect::new(
                    -recenter.0 + (((ent.pos.x - 1) - start_tile.x) * tile_size as i64) as i32 - x_pixels as i32,
                    -recenter.1 +(((ent.pos.y - 1) - start_tile.y) * tile_size as i64) as i32 - y_pixels as i32,
                    tile_size,
                    tile_size,
                );
                let _ = renderer.sdl.fill_rect(border_rect);
            }
            Ok(())
        });

        vec![]
    }

    fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::MouseMotion{mousestate, xrel, yrel, ..} => {
                if mousestate.left() {
                    self.viewport.add(-xrel, -yrel);
                }
            },
            Event::MouseWheel{y,..} => {
                self.viewport.zoom += y as f64 * -0.4;
                if self.viewport.zoom < 1.0 {
                    self.viewport.zoom = 1.0;
                } else if self.viewport.zoom > 5.0 {
                    self.viewport.zoom = 5.0;
                }
            },
            _ => {},
        }
    }
}
