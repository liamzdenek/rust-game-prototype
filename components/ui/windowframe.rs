use super::*;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use std::ops::DerefMut;

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
}

impl Viewframe for Windowframe{
    fn render(&mut self, renderer: &mut Renderer) {
        if self.no_border {
            //renderer.set_viewport(Some(Rect::new(30,30,700,700).unwrap().unwrap()));
            self.content.render(renderer);
        } else {
            println!("rendering window");
            let menu_size = 25;
            let border_size = 4;

            let window_rect = Rect::new(
                self.position.0 as i32, self.position.1 as i32,
                self.inner_size.0 + border_size * 2,
                self.inner_size.1 + border_size * 2 + menu_size,
            ).unwrap().unwrap();

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
}
