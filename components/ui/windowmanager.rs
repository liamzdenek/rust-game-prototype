use super::*;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

use std::collections::VecDeque;
/*
a window must be able to:
 draw itself
 draw its component parts
 define callback regions for its components and ensure they are routed properly
*/


pub struct RootFrame {
    root: Window,
    windows: Vec<Window>,
}

pub struct RootManager {
    frames: Vec<RenderRegion>,
    next_id: FrameId,
}

impl RootManager {
    pub fn new() -> Self {
        RootManager{
            frames: vec![],
            next_id: 0,
        }
    }
}

impl RootManager {
    pub fn create_window(&mut self, frame: Box<Frame>) -> Window {
        let rect = UIRect{
            x: 30,
            y: 30,
            w: 380,
            h: 175,
        };
        let frame_id = self.push_frame(RenderRegion::new(rect, frame));
        Window::new(frame_id)
    }
}

impl Manager for RootManager {
    fn push_frame(&mut self, mut frame: RenderRegion) -> FrameId {
        let needs_id = frame.id == None;
        let ret = if needs_id {
            frame.id = Some(self.next_id);
            self.next_id += 1;
            self.next_id - 1
        } else {
            frame.id.unwrap()
        };
        self.frames.push(frame);
        ret
    }
    fn borrow_frame_by_id(&mut self, id: FrameId) -> Option<&mut RenderRegion> {
        self.frames.iter_mut().find(|frame| frame.id == Some(id))
    }
    fn take_frame_by_id(&mut self, id: FrameId) -> Option<RenderRegion> {
        self.frames.iter().enumerate().find(|&(i, frame)| frame.id == Some(id))
            .and_then(|(i, _)| Some(i))
            .and_then(|i| Some(self.frames.swap_remove(i)))
    }
}

impl RootFrame {
    pub fn new(root: FrameId) -> Self {
        let mut root = Window::new(root);
        root.no_border = true;
        RootFrame{
            root: root,
            windows: vec![],
        }
    }

    fn get_full_rect(&mut self, renderer: &mut Renderer) -> UIRect {
        let full_size = if let Some(window) = renderer.window() {
            window.drawable_size()
        } else if let Some(surface) = renderer.surface() {
            surface.size()
        } else {
            unreachable!{}
        };

        UIRect{x: 0, y: 0, w: full_size.0, h: full_size.1}
    }

    pub fn push_window(&mut self, window: Window) {
        self.windows.push(window);
    }
    
    pub fn begin_render(&mut self, manager: &mut Manager, renderer: &mut Renderer) {

        let full_size = self.get_full_rect(renderer);
        //println!("FULL RECT: {:?}", full_size);

        #[derive(Debug)]
        enum WorkKind {
            Root,
            Window(usize),
            Frame(UIRect, FrameId)
        }

        let mut queue = VecDeque::new();  
      

        for (i, window) in self.windows.iter().enumerate() {
            queue.push_front(WorkKind::Window(i));
        }
        
        queue.push_front(WorkKind::Root);
       
        //println!("Qeuue at start: {:?}", queue);

        'frameloop: loop {
            let (parent_rect, new_frames): (UIRect, Vec<(UIRect, FrameId)>) = match queue.pop_front() {
                Some(WorkKind::Root) => {
                    (full_size.clone(), self.root.render(manager, full_size.clone(), renderer))
                },
                Some(WorkKind::Window(window_i)) => {
                    let size = self.windows[window_i].size.clone();
                    renderer.set_viewport(Some(size.clone().into()));
                    (size.clone(), self.windows[window_i].render(manager, size, renderer))
                },
                Some(WorkKind::Frame(rect, frame_id)) => {
                    match manager.take_frame_by_id(frame_id) {
                        Some(mut frame) => {
                            renderer.set_viewport(Some(rect.clone().into()));
                            let mut sent_rect = rect.clone();
                            sent_rect.x = 0;
                            sent_rect.y = 0;
                            let res = frame.frame.render(manager, sent_rect, renderer);
                            manager.push_frame(frame);
                            (rect, res)
                        }
                        None => {
                            println!("Attempted to render frame by id, but it couldn't be found. ID: {:?}", frame_id);
                            (UIRect::default(), vec![])
                        }
                    }
                },
                None => {
                    break 'frameloop;
                }
            };

            for (mut rect, frame) in new_frames {
                rect.x += parent_rect.x;
                rect.y += parent_rect.y;
                queue.push_front(WorkKind::Frame(rect, frame));
            }
        }
        renderer.set_viewport(None);
    }
}

