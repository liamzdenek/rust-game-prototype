use super::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::mouse::Mouse;

use std::any::Any;

pub struct Button {
    pub color: Color,
    pub bcast: Box<Broadcaster>,
}

impl Button {
    pub fn new(bcast: Box<Broadcaster>) -> Self {
        Button {
            color: sdl2::pixels::Color::RGB(0,0,0),
            bcast: bcast,
        }
    }
}

impl Frame for Button {
    fn render(&mut self, manager: &mut Manager, size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        renderer.sdl.draw_rect(
            size.clone().into()
        ).unwrap();

        vec![]
    }
    fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::MouseButtonDown{x, y, mouse_btn, ..} => {
                if mouse_btn == Mouse::Left {
                    self.bcast.broadcast();
                    println!("button got click");
                }
            },
            _ => {},
        }
    }
}
