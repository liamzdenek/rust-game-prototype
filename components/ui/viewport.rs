#[derive(Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport{
            //x:4.87,
            //y:10.09,
            x:0.0,
            y:0.0,
            zoom: 4.0,
        }
    }
}

impl Viewport {
    pub fn add(&mut self, xrel: f32, yrel: f32)  {
        self.x += (xrel as f32);// * self.zoom);
        self.y += (yrel as f32)// * self.zoom);
    }


    pub fn get_point_at_cursor(&self, size: (u32, u32), mouse_pos: (i32, i32)) -> (f32, f32) {
        let (px_tile_size, ogl_tile_size, ogl_start_ofs, start_tile, end_tile, focused_tile) = self.get_render_info(size);

        //focused_tile.0 as f32  
        (0.0,0.0)
    }

    pub fn update_zoom(&mut self, direction: f32, size: (u32, u32), mouse_pos: (i32, i32)) {
        println!("update zoom: {:?} {:?}", direction, mouse_pos);

        println!("point at cursor: {:?}", self.get_point_at_cursor(size, mouse_pos));
        self.zoom += -direction * 0.2;
        println!("zoom: {:?}", self.zoom);
        if self.zoom < 0.2 {
            self.zoom = 0.2;
        } else if self.zoom > 5.0 {
            self.zoom = 5.0;
        }
    }

    pub fn get_render_info(&self, size: (u32, u32)) -> (u32, (f32, f32), (f32, f32), (i32, i32), (i32, i32), (i32, i32)) {

        let viewport_size = 2.0;
        let tile_size = 1.0;
        let px_tile_size = (100.0 / self.zoom) as u32; 

        let mut num_tiles = (
            (size.0 / px_tile_size),
            (size.1 / px_tile_size),
        );

        let ogl_tile_size = (
            viewport_size / num_tiles.0 as f32,
            viewport_size / num_tiles.1 as f32,
            //(size.0 as f64 / viewport_size * ( tile_size / self.zoom ) as f64) as f32, 
            //(size.1 as f64 / viewport_size * ( tile_size / self.zoom ) as f64) as f32,
        );

        let focused_tile = (
            self.x as i32,
            self.y as i32,
        );

        let ogl_tile_ofs = (
            ogl_tile_size.0 * self.x.fract(),
            ogl_tile_size.1 * self.y.fract(),
        );

        // one extra on each side to ensure that theres no blank region around the borders
        // this cannot be done at the beginning since tile size must be computed without this
        // addition
        num_tiles = (
            num_tiles.0 + 2,
            num_tiles.1 + 2,
        );

        let start_tile = (
            focused_tile.0 - (num_tiles.0 as i32/ 2),
            focused_tile.1 - (num_tiles.1 as i32/ 2),
        );

        let end_tile = (
            start_tile.0 + num_tiles.0 as i32,
            start_tile.1 + num_tiles.1 as i32,
        );


        return (
            px_tile_size,
            ogl_tile_size,
            ogl_tile_ofs,
            start_tile,
            end_tile,
            focused_tile
        )
    }
}
