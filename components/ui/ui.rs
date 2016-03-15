use super::*;
use imgui::*;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;
use glium::program;
use glium::glutin;
use glium::DisplayBuild;

pub trait Renderer {
    fn render(&mut self, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame);
    fn handle_events(&mut self, events: Vec<Event>);
}

pub trait RendererBuilder {
    type O;
    fn build(&mut self, im_gui: &mut ImGui, display: &mut GlutinFacade) -> Self::O;
}

pub struct AppData {
    pub background: Box<Renderer>
}

pub struct UI {
    pub clear_color: (f32, f32, f32, f32),
}

impl UI {
    pub fn new() -> Self {
        UI{
            clear_color: (0.2, 0.2, 0.2, 1.0),
        }
    }

    pub fn run<T: Renderer + 'static>(&mut self, mut root: Box<RendererBuilder<O=T>>) {
        
        let mut display = glutin::WindowBuilder::new()
            .with_dimensions(1920,1080)
            .build_glium()
            .unwrap();
        let mut tex_cache = TexCache::new(&mut display);
        let mut support = Support::init(display);
        let mut app_data = AppData{
            background: Box::new(root.build(&mut support.imgui, &mut support.display)),
        };
        let mut open = true;

        'mainloop: loop {
            support.render(self.clear_color, |mut frame, mut display, ui| {
                app_data.background.render(&mut tex_cache, display, frame);
                
                ui.window(im_str!("Hello world"))
                    .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                    .title_bar(false)
                    .movable(false)
                    .resizable(false)
                    .build(|| {
                        ui.text(im_str!("Hello world!"));
                        ui.text(im_str!("This...is...imgui-rs!"));
                        ui.separator();
                        let mouse_pos = ui.imgui().mouse_pos();
                        ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                    });
                ui.show_metrics_window(&mut open);
                ui.show_test_window(&mut open);
            });
            let (events, active) = support.update_events();
            app_data.background.handle_events(events);
            if !active || !open {
                break 'mainloop;
            }
        }
    }
}
