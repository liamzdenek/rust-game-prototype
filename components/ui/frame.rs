use super::*;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::rect::Rect;
use sdl2::event::Event;

use std::convert::From;

pub type FrameId = usize;

pub struct Renderer<'a> {
    pub sdl: SdlRenderer<'a>,
}

pub struct RenderRegion {
    pub id: Option<FrameId>,
    //pub region: UIRect,

    pub viewport: Option<UIRect>,
    pub frame: Box<Frame>,
}

impl RenderRegion {
    pub fn new(rect: UIRect, frame: Box<Frame>) -> Self {
        RenderRegion{
            id: None,
            viewport: None,
            frame: frame,
        }
    }
}

#[derive(Default,Clone,Debug)]
pub struct UIRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl UIRect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.x < x &&
        self.y < y &&
        self.x + self.w as i32 > x &&
        self.y + self.h as i32 > y
    }
}

impl Into<Rect> for UIRect {
    fn into(self) -> Rect {
        Rect::new(self.x,self.y,self.w,self.h)
    }
}

impl From<Rect> for UIRect {
    fn from(val: Rect) -> UIRect {
        UIRect {
            x: val.x(),
            y: val.y(),
            h: val.height(),
            w: val.width(),
        }
    }
}

pub trait Frame {
    fn render(&mut self, &mut Manager, UIRect, &mut Renderer) -> Vec<(UIRect, FrameId)>;
    fn handle_event(&mut self, Event); 
}

pub trait Manager {
    fn push_frame(&mut self, RenderRegion) -> FrameId;
    fn borrow_frame_by_id(&mut self, FrameId) -> Option<&mut RenderRegion>;
    fn take_frame_by_id(&mut self, FrameId) -> Option<RenderRegion>;
}

