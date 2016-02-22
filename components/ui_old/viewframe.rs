use sdl2::render::Renderer;
use sdl2::event::Event;

pub trait Viewframe {
    fn render(&mut self, &mut Renderer);
    fn handle_event(&mut self, Event);
}
