use universe::GridKey;
use universe;
use physics::PhysicsHooks;
use universe::Result;
use rustc_serialize::*;
use universe::Pixel;
use gamerules::Position;
use gamerules::ReactivationRule;
use universe::Universe;
use physics::RichPixel;
use std::result;
/*
pub enum Behavior {
    Set(Pixel),
    MoveSelf(Position),
    DeactivateSelf(Vec<ReactivationRule>),
}*/

pub type Behavior = Box<InnerBehavior>;

//#[derive(RustcDecodable,RustcEncodable)]
pub trait InnerBehavior {
    fn get_lock_requirements(&self, phys: &PhysicsHooks) -> Vec<GridKey>;
    fn perform(&self, phys: &mut PhysicsHooks) -> Result<()>;
}

impl Encodable for Behavior {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        panic!("todo 1");
    }
}

impl Decodable for Behavior {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        panic!("todo 2"); 
    }
}

pub struct Set {
    src: Position,
    p: Pixel,
}

impl Set {
    pub fn new(src: Position, p: Pixel) -> Behavior {
        Box::new(Set{
            src: src,
            p: p,
        })
    }
}

impl InnerBehavior for Set {
    fn get_lock_requirements(&self, phys: &PhysicsHooks) -> Vec<GridKey> {
        vec![phys.borrow_universe().grid.derive_pos(self.src.x, self.src.y).2]
    }
    fn perform(&self, phys: &mut PhysicsHooks) -> Result<()> {
        phys.set_pixel(self.src.clone(), self.p.clone())
    }
}

pub struct MoveSelf {
    src: Position,
    dst: Position,
}

impl MoveSelf {
    pub fn new(src: Position, dst: Position) -> Behavior {
        Box::new(MoveSelf{
            src: src,
            dst: dst,
        })
    }
}

impl InnerBehavior for MoveSelf {
    fn get_lock_requirements(&self, phys: &PhysicsHooks) -> Vec<GridKey> {
        panic!("todo 3");
    }
    fn perform(&self, phys: &mut PhysicsHooks) -> Result<()> {
        panic!("todo 6");
    }
}

pub struct DeactivateSelf {
    src: Position,
    rules: Vec<ReactivationRule>,
}

impl DeactivateSelf {
    pub fn new(src: Position, rules: Vec<ReactivationRule>) -> Behavior {
        Box::new(DeactivateSelf{
            src: src,
            rules: rules,
        })
    }
}

impl InnerBehavior for DeactivateSelf {
    fn get_lock_requirements(&self, phys: &PhysicsHooks) -> Vec<GridKey> {
        panic!("todo 4");
    }
    fn perform(&self, phys: &mut PhysicsHooks) -> Result<()> {
        panic!("todo 7");
    }
}
