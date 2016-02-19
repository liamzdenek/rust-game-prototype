use super::*;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

pub struct WindowManager {
    root: Windowframe,
    windows: Vec<Windowframe>
}

impl WindowManager {
    pub fn new(root: Box<Viewframe>) -> Self {
        let mut root = Windowframe::new(root);
        root.no_border = true;
        WindowManager{
            root: root,
            windows: vec![],
        }
    }

    pub fn push_window(&mut self, window: Windowframe) {
        self.windows.push(window);
    }
}

impl Viewframe for WindowManager {
    fn render<'a>(&mut self, renderer: &mut Renderer) {
        /*
        renderer.set_viewport(Some(Rect::new(
            300, 300,
            500, 500
        ).unwrap().unwrap()));
        renderer.set_scale(0.2, 0.2);
        */
        self.root.render(renderer);
        renderer.set_viewport(None);
        for window in self.windows.iter_mut() {
            window.render(renderer);
            renderer.set_viewport(None);
        }
    }
}
