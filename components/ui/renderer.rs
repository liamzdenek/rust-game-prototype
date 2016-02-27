use sdl2::render::Renderer as SdlRenderer;
use sdl2_ttf::{Sdl2TtfContext, Font};
use std::collections::HashMap;

pub struct Renderer<'a> {
    pub sdl: SdlRenderer<'a>,
    pub ttf: Sdl2TtfContext,
    pub fonts: HashMap<(String, u16), Font>,
}

impl<'a> Renderer<'a> {
    pub fn new(sdl: SdlRenderer<'a>, ttf: Sdl2TtfContext) -> Self {
        Renderer{
            sdl:sdl,
            ttf:ttf,
            fonts: HashMap::new(),
        }
    }

    pub fn load_font(&mut self, name: String, path: String, size: u16) {
        let key = (name, size);
        if let Some(_) =  self.fonts.get(&key) {
            return;
        }
        use std::path::Path;
        self.fonts.insert(key, self.ttf.load_font(Path::new(&path), size).unwrap());
    }

    pub fn borrow_font(&mut self, name: String, size: u16) -> Option<&mut Font> {
        self.fonts.get_mut(&(name,size))
    }
}
