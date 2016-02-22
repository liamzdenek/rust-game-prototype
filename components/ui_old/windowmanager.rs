use super::*;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::event::Event;

enum FocusState {
    Apply,
    ApplyAndLock,
    Release,
    None,
}

enum FocusMutation {
    Ignore,
    Calculate{ apply: FocusState },
}

#[derive(Clone, Debug)]
pub enum FrameOption {
    SomeWindow(usize),
    SomeRoot,
    None,
}

pub struct WindowManager {
    root: Windowframe,
    windows: Vec<Windowframe>,
    focus_lock: FrameOption,
}

impl WindowManager {
    pub fn new(root: Box<Viewframe>) -> Self {
        let mut root = Windowframe::new(root);
        root.no_border = true;
        WindowManager{
            root: root,
            windows: vec![],
            focus_lock: FrameOption::None,
        }
    }

    pub fn push_window(&mut self, window: Windowframe) {
        self.windows.push(window);
    }

    fn get_locked(&mut self, x: u32, y: u32) -> FrameOption{
        match self.focus_lock {
            FrameOption::None => {},
            _ => {
                return self.focus_lock.clone();
            }
        }
        for (window_i, window) in self.windows.iter().enumerate() {
            if window.contains(x, y) {
                return FrameOption::SomeWindow(window_i);
            }
        }
        FrameOption::SomeRoot
    }
    
    fn push_event(&mut self, x: i32, y: i32, ev: Event, focus: FocusMutation) {
        match focus {
            FocusMutation::Ignore => {
                self.root.handle_event(ev);
            },
            FocusMutation::Calculate{apply} => {
                let by_coords = self.get_locked(x as u32, y as u32);
                match &by_coords {
                    &FrameOption::SomeWindow(window_i) => {
                        self.windows[window_i].handle_event(ev);
                        match apply {
                            FocusState::Apply => {
                                self.focus(window_i);
                            },
                            FocusState::ApplyAndLock => {
                                self.focus(window_i);
                                self.lock(by_coords);
                            },
                            FocusState::Release => {
                                self.unlock();
                            },
                            FocusState::None => {},
                        }
                    },
                    &FrameOption::SomeRoot | &FrameOption::None => {
                        self.root.handle_event(ev);
                        match apply {
                            FocusState::ApplyAndLock => {
                                self.lock(FrameOption::SomeRoot);
                            },
                            FocusState::Release => {
                                self.unlock();
                            },
                            _ => {},
                        }
                    }

                }
            },
        }
    }

    fn focus(&mut self, window_i: usize) {
        
    }

    fn lock(&mut self, locked: FrameOption) {
        println!("LOCKING TO: {:?}", locked);
        self.focus_lock = locked;
    }

    fn unlock(&mut self) {
        println!("UNLOCKING");
        self.focus_lock = FrameOption::None;
    }

    //fn lock_to_window(&mut self, 
}

impl Viewframe for WindowManager {
    fn render(&mut self, renderer: &mut Renderer) {
        self.root.render(renderer);
        renderer.set_viewport(None);
        for window in self.windows.iter_mut() {
            window.render(renderer);
            renderer.set_viewport(None);
        }
    }

    fn handle_event(&mut self, ev: Event) {
        match &ev {
            &Event::MouseButtonDown{x, y, ..} => {
                self.push_event(x, y, ev, FocusMutation::Calculate{apply: FocusState::Apply})
            },
            &Event::MouseButtonUp{x, y, ..} => {
                self.push_event(x, y, ev, FocusMutation::Calculate{apply: FocusState::Release})
            },
            &Event::MouseMotion{x, y, ..} => {
                self.push_event(x, y, ev, FocusMutation::Calculate{apply: FocusState::None})
            },
            /*Event::MouseMotion{mousestate, xrel, yrel, ..} => {
                if mousestate.left() {
                    //self.viewport.add(-xrel, -yrel);
                }
            },*/
            _ => {
                println!("WINMAN: Unhandled event: {:?}", ev);
            }
        }
    }
}
