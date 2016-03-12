use super::*;
use imgui::*;
use glium::backend::glutin_backend::GlutinFacade;

pub trait Renderer {
    fn render(&mut self, display: &mut GlutinFacade, frame: &mut Frame);
}

pub trait RendererBuilder {
    type O;
    fn build(&mut self, display: &mut GlutinFacade) -> Self::O;
}

pub struct AppData {
    pub background: Box<Renderer>
}

pub struct UI {
    pub support: Support,
    pub clear_color: (f32, f32, f32, f32),
}

impl UI {
    pub fn new() -> Self {
        UI{
            support: Support::init(),
            clear_color: (0.2, 0.2, 0.2, 1.0),
            
        }
    }

    pub fn run<T: Renderer + 'static>(&mut self, mut root: Box<RendererBuilder<O=T>>) {
        let built = root.build(&mut self.support.display);
        let mut app_data = AppData{
            background: Box::new(built),
        };
        let mut open = true;
        'mainloop: loop {
            self.support.render(self.clear_color, |mut frame, mut display, ui| {
                app_data.background.render(display, frame);
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
                //ui.show_test_window(&mut open);
            });
            let active = self.support.update_events();
            if !active || !open {
                break 'mainloop;
            }
        }
    }
}
