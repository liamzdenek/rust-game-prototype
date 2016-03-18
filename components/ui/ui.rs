use super::*;
use imgui::*;
use glium::glutin::Event;
use glium::program;
use glium::glutin;
use glium::DisplayBuild;

pub trait Renderer: ImguiRenderer {
    fn render(&mut self, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame);
    fn handle_events(&mut self, events: Vec<Event>);
}

pub trait RendererBuilder {
    type O;
    fn build(&mut self, im_gui: &mut ImGui, display: &mut GlutinFacade) -> Self::O;
}

pub struct ImguiRendererEntry {
    pub renderer: Box<ImguiRenderer>,
}

pub trait ImguiRenderer {
    fn render_ui<'ui>(&mut self, ui: &Ui<'ui>, app_data: &mut AppData, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame);
}

pub struct AppData {
    arbitrary: bool,
    //pub background: Box<Renderer>
}

pub struct UI {
    pub clear_color: (f32, f32, f32, f32),
    pub windows: Vec<ImguiRendererEntry>,
}

impl UI {
    pub fn new() -> Self {
        UI{
            clear_color: (0.2, 0.2, 0.2, 1.0),
            windows: vec![],
        }
    }

    pub fn run<T: Renderer + 'static>(&mut self, mut root: Box<RendererBuilder<O=T>>) {
        
        let mut display = glutin::WindowBuilder::new()
            .with_dimensions(1920,1080)
            .build_glium()
            .unwrap();
        let mut tex_cache = TexCache::new(&mut display);
        let mut support = Support::init(display);
        let mut background = root.build(&mut support.imgui, &mut support.display);
        let mut app_data = AppData{
            arbitrary: true,
        };
        let mut open = true;

        let mut windows: Vec<_> = self.windows.drain(..).collect();

        'mainloop: loop {
            support.render(self.clear_color, |mut frame, mut display, ui| {
                background.render(&mut tex_cache, display, frame);
              
                background.render_ui(ui, &mut app_data, &mut tex_cache, display, frame);

                for window in windows.iter_mut() {
                    window.renderer.render_ui(ui, &mut app_data, &mut tex_cache, display, frame);
                }
                /* 
                ui.show_metrics_window(&mut open);
                ui.show_test_window(&mut open);
                */
            });
            let (events, active) = support.update_events();
            background.handle_events(events);
            if !active || !open {
                break 'mainloop;
            }
        }
    }
}
