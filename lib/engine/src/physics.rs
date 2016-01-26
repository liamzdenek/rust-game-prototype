use universe;
use universe::{Result,GridKey,Pixel};
use gamerules::{GameRules,Position};
use behavior::Behavior;
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(RustcEncodable,RustcDecodable)]
pub struct RichPixel {
    pub x: i64,
    pub y: i64,
    pub pixel: universe::Pixel,
}

impl RichPixel {
    pub fn pos(&self) -> Position {
        Position{
            x: self.x,
            y: self.y,
        }
    }
    pub fn relative_pos(&self, d_x: i64, d_y: i64) -> Position {
        Position{
            x: self.x + d_x,
            y: self.y + d_y,
        }
    }
}

pub struct StandardPhysics {
    univ: universe::Universe,
    data: StandardPhysicsData,
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct StandardPhysicsData {
    active_pixels: Vec<Position>,
    work_queue: Vec<Vec<Behavior>>,
}

impl StandardPhysicsData {
    pub fn new() -> Self {
        StandardPhysicsData{
            active_pixels: vec![],
            work_queue: vec![],
            //grid_work_queue: HashMap::new(),
        }
    }
    
    pub fn push_active_pixel(&mut self, pixel: RichPixel) -> Result<()>{
        let search = self.active_pixels.iter()
            .find(|t| pixel.pos().eq(t)).is_none();
        if search {
            self.active_pixels.push(pixel.pos());
        }
        Ok(())
    }
}

pub trait PhysicsHooks {
    fn borrow_universe(&self) -> &universe::Universe; 
    fn set_pixel(&mut self, pos: &Position, pixel: Pixel) -> Result<()>;
    fn get_pixel(&mut self, pos: &Position) -> Result<Pixel>;
}

impl PhysicsHooks for StandardPhysics {
    fn borrow_universe(&self) -> &universe::Universe {
        &self.univ
    }
    fn set_pixel(&mut self, pos: &Position, pixel: Pixel) -> Result<()> {
        println!("Handling set_pixel in physhooks, {},{} = {:?}", pos.x, pos.y, pixel);
        self.univ.set_pixel(pos.x, pos.y, pixel.clone()).and_then(|_| {
            self.data.push_active_pixel(RichPixel{
                x: pos.x,
                y: pos.y,
                pixel: pixel,
            })
        })
    }
    fn get_pixel(&mut self, pos: &Position) -> Result<Pixel> {
        self.univ.get_pixel(pos.x, pos.y)        
    }
}

impl StandardPhysics {
    pub fn create(universe: universe::Universe, rules: &mut Box<GameRules>) -> Result<Self> {
        let mut universe = universe;
        let data = StandardPhysicsData::new();

        let mut res = StandardPhysics{
            data: data,
            univ: universe,
        };


        let behaviors = try!(rules.as_mut().new_game());
        try!(res.handle_rich_behaviors(behaviors));

        Ok(res)
    }

    pub fn load(universe: universe::Universe, rules: &mut Box<GameRules>) -> Result<Self> {
        let mut universe = universe;
        
        let mut res = StandardPhysics{
            data: try!(universe.get_json("physics_data.json".to_string())),
            univ: universe,
        };
        
        let behaviors = try!(rules.as_mut().load_game());
        try!(res.handle_rich_behaviors(behaviors));

        Ok(res)
    }

    pub fn handle_rich_behaviors(&mut self, behaviors: Vec<Behavior>) -> Result<()> {
        self.data.work_queue.push(behaviors);
        Ok(())
    }
   
    pub fn flush_queue(&mut self) {
        let drain = self.data.work_queue.drain(..).collect::<Vec<_>>();
        'outer: for behavior_list in drain {
            let mut err: Option<universe::UniverseError> = None;
            for behavior in behavior_list {
                let result = behavior.perform(self);
                match result {
                    Ok(_) => {
                        continue 'outer; 
                    }
                    Err(e) => {
                        err = Some(e);
                        continue;
                    }
                }
            }
            match err {
                None => {
                    panic!("A rich behavior cannot fail and not provide an error, this should not happen");
                },
                Some(e) => {
                    println!("Couldn't complete rich behaviors, encountered errors on all tasks. {:?}", e);
                }
            }
        }
    }

    pub fn run_tick(&mut self, rules: &mut Box<GameRules>) -> Result<()>{
        self.flush_queue();

        try!(rules.pre_tick());
        for pos in self.data.active_pixels.iter() {
            try!(rules.handle_pixel_tick(self, pos));
        }
        try!(rules.post_tick());

        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        Err(universe::UniverseError::Unimplemented("StandardPhysics.save"))
    }
}
