use super::*;
use imgui::*;
use tick_traits::tick_thread::Tick; 

pub struct TimeControls {
    tick: Tick,
}

pub enum TimeSpeed {
    Paused,
    Slow,
    Medium,
    Fast,
    Unlimited,
}

impl TimeControls {
    pub fn new(tick: Tick) -> Self {
        TimeControls{
            tick: tick,
        }
    }


    pub fn set_speed(&mut self, speed: TimeSpeed) {
        let ms = match speed {
            TimeSpeed::Paused => 0,
            TimeSpeed::Slow => 2500,
            TimeSpeed::Medium => 1000,
            TimeSpeed::Fast => 250,
            TimeSpeed::Unlimited => 1,
        };
        self.tick.set_speed(ms);
    }
}

impl ImguiRenderer for TimeControls {
    fn render<'ui>(&mut self, ui: &Ui<'ui>, app_data: &mut AppData, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            .build(|| {
                if ui.small_button(im_str!("Pause")) {
                    self.set_speed(TimeSpeed::Paused);
                }
                if ui.small_button(im_str!(">")) {
                    self.set_speed(TimeSpeed::Slow);
                }
                if ui.small_button(im_str!(">>")) {
                    self.set_speed(TimeSpeed::Medium);
                }
                if ui.small_button(im_str!(">>>")) {
                    self.set_speed(TimeSpeed::Fast);
                }
                if ui.small_button(im_str!(">>>>")) {
                    self.set_speed(TimeSpeed::Unlimited);
                }
            });
    }
}
