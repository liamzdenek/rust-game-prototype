use super::*;
use imgui::*;
use tick_traits::tick_thread::Tick; 

pub struct TimeControls {
    tick: Tick,
    curspeed: TimeSpeed,
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
            curspeed: TimeSpeed::Medium,
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
        self.curspeed = speed;
        self.tick.set_speed(ms);
    }
}

impl ImguiRenderer for TimeControls {
    fn render_ui<'ui>(&mut self, ui: &Ui<'ui>, app_data: &mut AppData, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {
        let mut opened = true;
        ui.window(im_str!("Time"))
            .size((200.0, 50.0), ImGuiSetCond_FirstUseEver)
            .opened(&mut opened)
            //.title_bar(false)
            //.movable(false)
            .resizable(false)
            .build(|| {
                ui.same_line(0.0);
                if ui.small_button(im_str!("||")) {
                    self.set_speed(TimeSpeed::Paused);
                }
                ui.same_line(0.0);
                if ui.small_button(im_str!(">")) {
                    self.set_speed(TimeSpeed::Slow);
                }
                ui.same_line(0.0);
                if ui.small_button(im_str!(">>")) {
                    self.set_speed(TimeSpeed::Medium);
                }
                ui.same_line(0.0);
                if ui.small_button(im_str!(">>>")) {
                    self.set_speed(TimeSpeed::Fast);
                }
                ui.same_line(0.0);
                if ui.small_button(im_str!(">>>>")) {
                    self.set_speed(TimeSpeed::Unlimited);
                }
            });

        if !opened {

        }
    }
}
