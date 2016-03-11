#[derive(Clone)]
pub struct Viewport {
    pub x: i64,
    pub y: i64,
    pub tile_size: i64,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport{
            x:0,// 487,
            y:0,// 309,
            tile_size: 100,
            zoom: 4.0,
        }
    }
}

impl Viewport {
    pub fn add(&mut self, xrel: i32, yrel: i32)  {
        self.x += (xrel as f32 * self.zoom) as i64;
        self.y += (yrel as f32 * self.zoom) as i64;
    }

    pub fn get_render_info(&self, size: (u32, u32)) -> (f32, (u32,u32)) {

        println!("size: {:?}", size);
        let tile_size = self.tile_size as f32 / self.zoom;
        (
            self.zoom,
            (
                (size.0 as f32 / tile_size) as u32,
                (size.1 as f32 / tile_size) as u32, 
            )
        )
        /*
        let tile_size = self.tile_size as f64 / self.zoom;
        (
            tile_size as u32,
            self.x / self.tile_size as i64,
            self.y / self.tile_size as i64,
            ((self.x % self.tile_size) as f64 / self.zoom) as i64,
            ((self.y % self.tile_size) as f64 / self.zoom) as i64,
        )
        */
    }
}
