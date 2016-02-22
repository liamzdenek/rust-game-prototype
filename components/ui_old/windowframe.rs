use super::*;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;

use std::ops::DerefMut;

pub trait WindowElement {
    
}

pub struct Windowframe {
    pub no_border: bool,
    pub title: String,
    pub content: Box<Viewframe>,
    pub position: (u32, u32),
    pub inner_size: (u32, u32),
}

impl Windowframe {
    pub fn new(content: Box<Viewframe>) -> Self {
        Windowframe{
            no_border: false,
            title: "Untitled Window".to_string(),
            content: content,
            inner_size: (380,175),
            position: (30, 30),
        }
    }

    pub fn get_window_details(&self) -> (u32, u32, Rect) {
        let menu_size = 25;
        let border_size = 4;
        let window_rect = Rect::new(
            self.position.0 as i32, self.position.1 as i32,
            self.inner_size.0 + border_size * 2,
            self.inner_size.1 + border_size * 2 + menu_size,
        ).unwrap().unwrap();

        (menu_size, border_size, window_rect)
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        let (_, _, window_rect) = self.get_window_details();
        window_rect.x() <= x as i32 &&
        window_rect.y() <= y as i32 &&
        window_rect.x() + window_rect.width() as i32 >= x  as i32&&
        window_rect.y() + window_rect.height() as i32 >= y as i32
    }
}

impl Viewframe for Windowframe{
    fn render(&mut self, renderer: &mut Renderer) {
        if self.no_border {
            //renderer.set_viewport(Some(Rect::new(30,30,700,700).unwrap().unwrap()));
            self.content.render(renderer);
        } else {
            //println!("rendering window");

            let (menu_size, border_size, window_rect) = self.get_window_details();

            renderer.set_viewport(Some(window_rect));
            renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.fill_rect(
                Rect::new(0,0,window_rect.width(),window_rect.height()).unwrap().unwrap(),
            );

            let inner_rect = Rect::new(
                (self.position.0 + border_size) as i32,
                (self.position.1 + border_size+ menu_size) as i32,
                self.inner_size.0,
                self.inner_size.1,
            ).unwrap().unwrap();
            
            renderer.set_viewport(Some(inner_rect));
            self.content.render(renderer);
        }
    }

    fn handle_event(&mut self, ev: Event) {
        //println!("WINFRAME Got event: {:?}",  ev);
    }
}
