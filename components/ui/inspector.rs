use super::*;
use backend_traits::environment_thread::{LocalEntityData};

pub enum InspectorFocus {
    None,
    Entity(LocalEntityData),
}

pub struct Inspector {
    pub focus: InspectorFocus
}

impl Inspector {
    pub fn new() -> Self {
        Inspector {
            focus: InspectorFocus::None,
        }
    }
}

impl ImguiRenderer for Inspector {
    fn render_ui<'ui>(&mut self, ui: &Ui<'ui>, app_data: &mut AppData, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {
        let mut opened = true;
        let window_size = frame.get_dimensions();
        match self.focus {
            InspectorFocus::None => {

            },
            InspectorFocus::Entity(ref focus) => {
                let size = (380.0, 175.0);
                let pos = (
                    window_size.0 as f32 - size.0 - 10.0,
                    window_size.1 as f32 - size.1 - 10.0,
                );
                ui.window(im_str!("Inspector"))
                    .size((380.0,175.0), ImGuiSetCond_Always)
                    .position(pos, ImGuiSetCond_Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .scrollable(true)
                    .opened(&mut opened)
                    .build(|| {
                        ui.text_wrapped(im_str!("got focus: {:?}", focus)); 
                    });
            }
        }
        if !opened {
            self.focus = InspectorFocus::None;
        }
    }
}
