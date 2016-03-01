use super::*;

use sdl2::event::Event;

pub struct Splitter {
    pub entries: Vec<SplitterEntry>,
    pub is_vert: bool,
    pub depth: u32,
}

impl Splitter {
    pub fn new(entries: Vec<SplitterEntry>) -> Self {
        Splitter{
            entries: entries,
            is_vert: false,
            depth: 100,
        }
    }
}

impl Frame for Splitter {
    fn render(&mut self, manager: &mut Manager, size: UIRect, renderer: &mut Renderer) -> Vec<(UIRect, FrameId)> {
        let mut distance = 0;
        let is_vert = self.is_vert;
        let depth = self.depth;
        
        let mut ret = vec![];
        
        for entry in self.entries.iter() {
            match *entry {
                SplitterEntry::Static(frame_id, length) => {
                    let rect = if is_vert {
                        UIRect::new(0, distance, size.w, length)
                    } else {
                        UIRect::new(distance, 0, length, size.h)
                    };
                    distance += length as i32;

                    ret.push((rect.into(), frame_id));
                },
                SplitterEntry::Dynamic(frame_id) => {
                    let rect = if is_vert {
                        UIRect::new(0, distance, size.w, size.h - distance as u32)
                    } else {
                        UIRect::new(distance, 0, size.w - distance as u32, size.h)
                    };
                    ret.push((rect.into(), frame_id));
                },
            }
        }

        ret
    }
    
    fn handle_event(&mut self, ev: Event) {
        //unimplemented!();
    }
}

pub enum SplitterEntry {
    Static(FrameId, u32),
    Dynamic(FrameId)
}
