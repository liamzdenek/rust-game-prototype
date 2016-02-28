use super::*;
//use sdl2::render::Renderer;
use sdl2::event::Event;

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
    lookup_table: Vec<RegionLookupKind>,
    current_bound: StoredBound,
    last_mouse_pos: (i32, i32),
}

pub struct RootManager {
    frames: Vec<RenderRegion>,
    next_frame_id: FrameId,
    next_window_id: WindowId,
}

pub enum EventBindBehavior {
    Bind,
    Unbind,
    Unmodified
}

pub enum EventRoutingBehavior{
    Calculate(EventBindBehavior),
    AlwaysRoot,
}

#[derive(Clone)]
pub enum RegionLookupKind{
    None,
    Root,
    Window(UIRect, WindowId),
    Frame(UIRect, FrameId),
}

#[derive(Clone)]
pub enum StoredBound {
    None,
    Window(WindowId),
    Frame(FrameId),
}

impl RootManager {
    pub fn new() -> Self {
        RootManager{
            frames: vec![],
            next_frame_id: 0,
            next_window_id: 0,
        }
    }
}

impl RootManager {
    pub fn create_window(&mut self, frame: Box<Frame>) -> Window {
        let frame_id = self.push_frame(RenderRegion::new(frame));
        self.next_window_id += 1;
        Window::new(self.next_window_id-1, frame_id)
    }
}

impl Manager for RootManager {
    fn push_frame(&mut self, mut frame: RenderRegion) -> FrameId {
        let needs_id = frame.id == None;
        let ret = if needs_id {
            frame.id = Some(self.next_frame_id);
            self.next_frame_id += 1;
            self.next_frame_id - 1
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
        self.frames.iter().enumerate().find(|&(_, frame)| frame.id == Some(id))
            .and_then(|(i, _)| Some(i))
            .and_then(|i| Some(self.frames.swap_remove(i)))
    }
}

impl RootFrame {
    pub fn new(root: FrameId) -> Self {
        let mut root = Window::new(0, root);
        root.no_border = true;
        RootFrame{
            root: root,
            windows: vec![],
            lookup_table: vec![],
            current_bound: StoredBound::None,
            last_mouse_pos: (0,0),
        }
    }

    fn get_full_rect(&mut self, renderer: &mut Renderer) -> UIRect {
        let full_size = if let Some(window) = renderer.sdl.window() {
            window.drawable_size()
        } else if let Some(surface) = renderer.sdl.surface() {
            surface.size()
        } else {
            unreachable!{}
        };

        UIRect{x: 0, y: 0, w: full_size.0, h: full_size.1}
    }

    pub fn push_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn handle_event(&mut self, manager: &mut Manager, event: Event) {
        use EventRoutingBehavior::*;
        use EventBindBehavior::*;
        match event {
            Event::MouseButtonDown{x,y,..} => self.route_event(manager, (x,y), event, Calculate(Bind)),
            Event::MouseButtonUp{x,y,..} => self.route_event(manager, (x,y), event, Calculate(Unbind)),
            Event::MouseMotion{x,y,..} => self.route_event(manager, (x,y), event, Calculate(Unmodified)),
            Event::MouseWheel{..} => {
                let pos = self.last_mouse_pos.clone();
                self.route_event(manager, pos, event, Calculate(Unmodified))
            },
            _ => {
                println!("unhandled event: {:?}", event);
            }
        }
    }

    fn route_event(&mut self, manager: &mut Manager, point: (i32, i32), event: Event, behavior: EventRoutingBehavior) {
        match behavior {
            EventRoutingBehavior::AlwaysRoot => {
                unimplemented!();
            }
            EventRoutingBehavior::Calculate(behavior) => {
                // todo: swap this out for as ref?
                for entry in self.lookup_table.clone().into_iter().rev() {
                    let was_handled = match entry {
                        RegionLookupKind::None => { false },
                        RegionLookupKind::Root => {
                            println!("got root for event");
                            true
                        },
                        RegionLookupKind::Window(rect, window_id) => {
                            self.handle_bind(manager, rect, point, &behavior, StoredBound::Window(window_id), &event)
                        },
                        RegionLookupKind::Frame(rect, frame_id) => {
                            self.handle_bind(manager, rect, point, &behavior, StoredBound::Frame(frame_id), &event)
                        },
                    };
                    if was_handled {
                        break;
                    }
                }
            }
        }
    }

    pub fn handle_bind(&mut self, manager: &mut Manager, rect: UIRect, point: (i32, i32), behavior: &EventBindBehavior, new_bound: StoredBound,event:&Event) -> bool{
        let mut was_handled = false;

        self.last_mouse_pos = point;

        use EventBindBehavior::*;
        if let &Bind = behavior {
            if rect.contains(point.0,point.1) {
                self.current_bound = new_bound.clone();
            }
        }
        
        match self.current_bound {
            StoredBound::Window(window_id) => {
                if let Some(window) = self.borrow_window(window_id) {
                    was_handled = true;
                    window.handle_event(event.clone());
                }
            },
            StoredBound::Frame(frame_id) => {
                if let Some(frame) = manager.borrow_frame_by_id(frame_id) {
                    was_handled = true;
                    frame.frame.handle_event(event.clone());
                }
            },
            StoredBound::None => {
                match (rect.contains(point.0, point.1), new_bound) {
                    (true, StoredBound::Window(window_id)) => {
                        if let Some(window) = self.borrow_window(window_id) {
                            was_handled = true;
                            window.handle_event(event.clone());
                        }
                    },
                    (true, StoredBound::Frame(frame_id)) => {
                        if let Some(frame) = manager.borrow_frame_by_id(frame_id) {
                            was_handled = true;
                            frame.frame.handle_event(event.clone());
                        }
                    },
                    _ => {}
                }
            }
        }

        if let &Unbind = behavior {
            self.current_bound = StoredBound::None;
        }

        was_handled
    }

    pub fn borrow_window(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|window| { window.id == id })
    }

    pub fn destroy_window(&mut self, id: WindowId) {
        let windows = self.windows.clone();
        if let Some((i, _)) = windows.iter().enumerate().find(|&(_, window)| window.id == id) {
            self.windows.swap_remove(i);
        }
    }

    pub fn begin_render(&mut self, manager: &mut Manager, renderer: &mut Renderer) {
        let full_size = self.get_full_rect(renderer);
        //println!("FULL RECT: {:?}", full_size);

        #[derive(Debug)]
        enum WorkKind {
            Root,
            Window(WindowId),
            Frame(UIRect, FrameId)
        }

        let mut queue = VecDeque::new();  
      

        for window in self.windows.iter() {
            queue.push_front(WorkKind::Window(window.id));
        }
        
        queue.push_front(WorkKind::Root);
       
        let mut lookup_table = vec![];

        //println!("Qeuue at start: {:?}", queue);

        'frameloop: loop {
            let (parent_rect, lookup, new_frames): (UIRect, RegionLookupKind, Vec<(UIRect, FrameId)>) = match queue.pop_front() {
                Some(WorkKind::Root) => {
                    (full_size.clone(), RegionLookupKind::Root, self.root.render(manager, full_size.clone(), renderer))
                },
                Some(WorkKind::Window(window_id)) => {
                    let (ret, cur_manip) = if let Some(window) = self.borrow_window(window_id) {
                        let size = window.size.clone();
                        renderer.sdl.set_viewport(Some(size.clone().into()));
                        let ret = (size.clone(), RegionLookupKind::Window(size.clone(), window_id), window.render(manager, size, renderer));
                        (ret, window.cur_manipulation.clone())
                    } else {
                        ((UIRect::default(), RegionLookupKind::None, vec![]), WindowManipulationKind::None)
                    };
                    if let WindowManipulationKind::Close = cur_manip {
                        self.destroy_window(window_id);
                    }
                    ret
                },
                Some(WorkKind::Frame(rect, frame_id)) => {
                    match manager.take_frame_by_id(frame_id) {
                        Some(mut frame) => {
                            renderer.sdl.set_viewport(Some(rect.clone().into()));
                            let mut sent_rect = rect.clone();
                            sent_rect.x = 0;
                            sent_rect.y = 0;
                            let res = frame.frame.render(manager, sent_rect, renderer);
                            manager.push_frame(frame);
                            (rect.clone(), RegionLookupKind::Frame(rect, frame_id), res)
                        }
                        None => {
                            println!("Attempted to render frame by id, but it couldn't be found. ID: {:?}", frame_id);
                            (UIRect::default(), RegionLookupKind::None, vec![])
                        }
                    }
                },
                None => {
                    break 'frameloop;
                }
            };

            lookup_table.push(lookup);

            for (mut rect, frame) in new_frames {
                rect.x += parent_rect.x;
                rect.y += parent_rect.y;
                queue.push_front(WorkKind::Frame(rect, frame));
            }
        }

        self.lookup_table = lookup_table;
        renderer.sdl.set_viewport(None);
    }
}

