use super::*;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use std::cmp;

pub type WindowId = i32;

#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    pub no_border: bool,
    pub title: String,
    pub content: FrameId,
    pub size: UIRect,
    pub cur_manipulation: WindowManipulationKind,
    pub min_width: u32,
    pub min_height: u32,
}

#[derive(Clone)]
pub enum WindowManipulationKind {
    Move,
    Resize,
    Close,
    None,
}

impl Window {
    pub fn new(id: WindowId, content: FrameId) -> Self {
        Window{
            id: id,
            no_border: false,
            title: "Untitled Window".to_string(),
            content: content,
            size: UIRect{x: 30, y: 30, w: 380, h: 175},
            min_width: 380,
            min_height: 175,
            cur_manipulation: WindowManipulationKind::None,
        }
    }

    pub fn get_window_details(&self) -> (u32, u32, Rect, Rect) {
        let menu_size = 25;
        let border_size = 4;
        let window_rect = Rect::new(
            self.size.x as i32, self.size.y as i32,
            self.size.w,
            self.size.h,
        );


        let close_rect = Rect::new((window_rect.width()-menu_size) as i32, 0, menu_size, menu_size);

        (menu_size, border_size, window_rect, close_rect)
    }

    pub fn get_menu_rect(&self) -> Rect {
        let (menu_size, border_size, mut window_rect, _) = self.get_window_details();

        window_rect.set_height(menu_size+border_size);

        window_rect
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        let (_, _, window_rect, _) = self.get_window_details();
        window_rect.x() <= x as i32 &&
        window_rect.y() <= y as i32 &&
        window_rect.x() + window_rect.width() as i32 >= x  as i32&&
        window_rect.y() + window_rect.height() as i32 >= y as i32
    }
}

impl Frame for Window {
    fn render(&mut self, manager: &mut Manager, size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        let _ = manager; // make unused var warning shut up
        self.size = size.clone();
        if self.no_border {
            //renderer.set_viewport(Some(Rect::new(30,30,700,700).unwrap().unwrap()));
            vec![
                (size, self.content)
            ]
        } else {
            let (menu_size, border_size, window_rect, close_rect) = self.get_window_details();

            // draw background
            renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(46,65,114));
            renderer.sdl.fill_rect(
                Rect::new(0,0,window_rect.width(),window_rect.height()),
            ).unwrap();

            // draw border
            renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.sdl.draw_rect(
                Rect::new(0,0,window_rect.width(),window_rect.height()),
            ).unwrap();

            // draw close button
            renderer.sdl.draw_rect(close_rect).unwrap();
            
            // draw title
            let title_surf = &renderer.borrow_font("menu".to_string(), 64).unwrap().render(self.title.as_str())
                .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)).unwrap();
            let mut title_texture = renderer.sdl.create_texture_from_surface(title_surf).unwrap();

            let out_rect = get_scaled_rect(window_rect.width(), menu_size, Rect::new(
                border_size as i32,
                border_size as i32,
                title_texture.query().width,
                title_texture.query().height,
            ));

            renderer.sdl.copy(&mut title_texture, None, Some(out_rect));

            // send back info on rendering the contents
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

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown{x,y,mouse_btn,..} => {
                if mouse_btn == Mouse::Left {
                    let menu_rect = self.get_menu_rect();
                    let (_,_,window_rect,mut close_rect) = self.get_window_details();
                    let new_x = close_rect.x() + window_rect.x();
                    let new_y = close_rect.y() + window_rect.y();
                    close_rect.set_x(new_x);
                    close_rect.set_y(new_y);
                    //println!("close rect: {:?}", close_rect);
                    //println!("menu rect: {:?}", menu_rect);
                    self.cur_manipulation = if close_rect.contains((x,y)) {
                        WindowManipulationKind::Close
                    } else if menu_rect.contains((x,y)) {
                        WindowManipulationKind::Move
                    } else {
                        WindowManipulationKind::Resize
                    }
                }
            },
            Event::MouseMotion{xrel,yrel,mousestate,..} => {
                if mousestate.left() {
                    match self.cur_manipulation {
                        WindowManipulationKind::Move => {
                            self.size.x += xrel;
                            self.size.y += yrel;
                        },
                        WindowManipulationKind::Resize => {
                            self.size.w = cmp::max(self.min_width, ((self.size.w as i32) + xrel) as u32);
                            self.size.h = cmp::max(self.min_height, ((self.size.h as i32) + yrel) as u32);
                        }
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }
}

fn get_scaled_rect(dst_width: u32, dst_height: u32, src: Rect) -> Rect {
    let mut src = src;
    let scale_a = dst_width as f64/src.width() as f64;
    let scale_b = dst_height as f64/src.height() as f64;

    let scale = if scale_a < scale_b {
        scale_a
    }  else {
        scale_b
    };

    let new_w = src.width() as f64 * scale;
    let new_h = src.height() as f64 * scale;
    src.set_width(new_w as u32);
    src.set_height(new_h as u32);

    src
}
