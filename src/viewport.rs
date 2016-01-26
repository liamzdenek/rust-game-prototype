struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn empty() -> Self {
        Point{
            x: 0,
            y: 0,
        }
    }
}

struct Rect {
    x: i64,
    y: i64,
    w: u64,
    h: u64,
}

impl Rect {
    fn empty() -> Self {
        Rect{
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}

struct Sized {
    w: u64,
    h: u64,
}

impl Sized {
    fn empty() -> Self {
        Sized{
            w: 0,
            h: 0,
        }
    }
}

pub struct Viewport {
    focus: Point,
    window: Sized,
    zoom: u64,
}

impl Viewport {
    pub fn new() -> Self {
        Viewport{
            focus: Point::empty(),
            window:  Sized::empty(),
            zoom: 1,
        }
    }

    pub fn set_window_size(&mut self, w: u64, h: u64) {
        self.window.w = w;
        self.window.h = h;
    }

    pub fn get_render_params(&self) -> RenderParams {
        let grid_w = self.window.w / self.zoom;
        let grid_h = self.window.h / self.zoom;
        RenderParams{
            GridRegion: Rect{
                x: self.focus.x - ((grid_w / 2) as i64),
                y: self.focus.y - ((grid_h / 2) as i64),
                w: grid_w,
                h: grid_h,
            }
        }
    }
}

pub struct RenderParams {
    GridRegion: Rect,
}
