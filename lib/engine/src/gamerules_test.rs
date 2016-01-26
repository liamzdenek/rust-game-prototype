use universe;
use universe::Result;
use physics::RichPixel;
use gamerules::*;
use behavior::Behavior;
use behavior;
use universe::Pixel;

pub struct GameRulesTest {
    arbitrary: bool,
}

impl GameRulesTest {
    pub fn new() -> Self {
        return GameRulesTest{
            arbitrary: true,
        }
    }
}

impl GameRules for GameRulesTest {
    fn new_game(&mut self) -> Result<Vec<Behavior>>{
        Ok(vec![
            behavior::Set::new(Position::new(0,0),KindSand::new_pixel()),
        ])
    }
    fn load_game(&mut self) -> Result<Vec<Behavior>>{
        Ok(vec![])
    }
    fn pre_tick(&mut self) -> Result<Vec<Behavior>>{
        Ok(vec![])
    }
    fn post_tick(&mut self) -> Result<Vec<Behavior>>{
        Ok(vec![])
    }
    fn get_kinds(&mut self) -> Vec<Box<PhysicsKind>> {
        vec![
            Box::new(KindSand),
        ]
    }
    /*
    fn handle_pixel_tick(&mut self, pixel: &Position) -> Result<Vec<Behavior>>{
        Err(universe::UniverseError::Unimplemented("GameRulesTest.handle_pixel_tick"))
    }*/
}

pub struct KindSand;

impl KindSand {
    fn new_pixel() -> Pixel {
        Pixel::new(KindSand.kind())
    }
}

impl PhysicsKind for KindSand {
    fn kind(&mut self) -> String { 
        "sand".to_string()
    }
    fn handle_pixel(&mut self, pixel: &RichPixel) -> Result<Vec<Behavior>> {
        Ok(vec![
            behavior::MoveSelf::new(pixel.pos(), pixel.relative_pos(0, -1)),
            behavior::DeactivateSelf::new(pixel.pos(), vec![ReactivationRule::OnEdgeChange]),
        ])
    }
}
