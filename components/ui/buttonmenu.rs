use super::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::mem;
use std::collections::HashMap;

pub struct ButtonMenu {
    pub background: Color,
    pub buttons: Vec<ButtonKind>,
}

pub enum StoredButton {
    Button(Button),
    Frame(FrameId),
    None,
}

pub enum ButtonKind {
    Button(StoredButton),
    Spacer(u32),
}

impl ButtonMenu {
    pub fn new(buttons: Vec<ButtonKind>) -> Self {
        ButtonMenu{
            background: Color::RGB(255,100,100),
            buttons: buttons,
        }
    }
}

impl Frame for ButtonMenu {
    fn render(&mut self, manager: &mut Manager, size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        let is_vert = size.w < size.h;

        let mut offset = 0;
        let mut ret = vec![];

        renderer.sdl.set_draw_color(sdl2::pixels::Color::RGB(255, 70, 70));
        renderer.sdl.fill_rect(
            size.clone().into(),
        ).unwrap();

        for (next_i, next) in self.buttons.iter_mut().enumerate() {
            match next {
                &mut ButtonKind::Button(ref mut stored_button) => {
                    match *stored_button {
                        StoredButton::Button(_) => {
                            let button = mem::replace(stored_button, StoredButton::None);
                            if let StoredButton::Button(button) = button {
                                let frame_id = manager.push_frame(RenderRegion::new(Box::new(button)));
                                *stored_button = StoredButton::Frame(frame_id);
                            } else {
                                mem::replace(stored_button, button);
                            }
                        }
                        _=> {

                        }
                    }

                    if let &mut StoredButton::Frame(frame_id) = stored_button {
                        let rect = if is_vert {
                            UIRect::new(0, offset, size.w, size.w)
                        } else {
                            UIRect::new(offset, 0, size.h, size.h)
                        };
                        offset += rect.w as i32;
                        ret.push((rect, frame_id));
                    }

                },
                &mut ButtonKind::Spacer(space) => {
                    offset += space as i32;
                }
            }
        }
        
        ret
    }
    
    fn handle_event(&mut self, ev: Event) {
        //unimplemented!();
    }
}
