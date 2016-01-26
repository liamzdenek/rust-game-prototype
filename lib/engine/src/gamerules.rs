use universe;
use universe::Result;
use physics::PhysicsHooks;
use physics::RichPixel;
use universe::Pixel;
use behavior::Behavior;

pub trait GameRules {
    fn new_game(&mut self) -> Result<Vec<Behavior>>;
    fn load_game(&mut self) -> Result<Vec<Behavior>>;
    fn pre_tick(&mut self) -> Result<Vec<Behavior>>;
    fn post_tick(&mut self) -> Result<Vec<Behavior>>;
    fn get_kinds(&mut self) -> Vec<Box<PhysicsKind>>;
    fn handle_pixel_tick(&mut self, hooks: &PhysicsHooks, pos: &Position) -> Result<Vec<Behavior>> {
        let kinds = self.get_kinds();

        let pixel = try!(hooks.get_pixel(pos));

        kinds.iter().find(|t| t.kind() == pixel.kind).and_then(|kind|{
            println!("Got kind: {}", kind.kind());
            Ok(())
        });

        Ok(vec![])

        //self.get_pixel(

        
        //kinds.iter().find(|t| t.kind() == 
    }
}

#[derive(RustcDecodable,RustcEncodable,Clone,Eq,PartialEq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position{
            x: x,
            y: y,
        }
    }
}

#[derive(RustcDecodable,RustcEncodable)]
pub enum ReactivationRule {
    //OnGetPixel, // when someone calls get_pixel with this coordinate
    OnEdgeChange, // when an adjacent (+1/-1 any direction) pixel performs MoveSelf behavior
    //OnEdgeActivation, // When an adjacent pixel becomes active
    //OnEdgeDeactivation, // When an adjacent pixel becomes inactive
    //OnTimer(u64), // After a certain number of ticks
}

pub trait PhysicsKind {
    fn kind(&mut self) -> String;
    fn handle_pixel(&mut self, pixel: &RichPixel) -> Result<Vec<Behavior>>;
        
}
