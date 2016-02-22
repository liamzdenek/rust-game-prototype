use super::*;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use std::convert::From;

pub struct Window {
    pub no_border: bool,
    pub title: String,
    pub content: FrameId,
    pub size: UIRect,
}

impl Window {
    pub fn new(content: FrameId) -> Self {
        Window{
            no_border: false,
            title: "Untitled Window".to_string(),
            content: content,
            size: UIRect{x: 30, y: 30, w: 380, h: 175},
        }
    }

    pub fn get_window_details(&self) -> (u32, u32, Rect) {
        let menu_size = 25;
        let border_size = 4;
        let window_rect = Rect::new(
            self.size.x as i32, self.size.y as i32,
            self.size.w,
            self.size.h,
        );

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

impl Frame for Window {
    fn render(&mut self, manager: &mut Manager, size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        self.size = size.clone();
        if self.no_border {
            //renderer.set_viewport(Some(Rect::new(30,30,700,700).unwrap().unwrap()));
            vec![
                (size, self.content)
            ]
        } else {
            let (menu_size, border_size, window_rect) = self.get_window_details();

            //println!("got window rect: {:?}", window_rect);

            //renderer.set_viewport(Some(window_rect));
            renderer.set_draw_color(sdl2::pixels::Color::RGB(46,65,114));
            renderer.fill_rect(
                Rect::new(0,0,window_rect.width(),window_rect.height()),
            );

            let inner_rect = UIRect{
                x: border_size as i32,
                y: (border_size+ menu_size) as i32,
                w: (self.size.w - border_size * 2) as u32,
                h: (self.size.h - border_size * 2 - menu_size) as u32,
            };
            
            vec![
                (inner_rect, self.content)
            ]
        }
    }
}
