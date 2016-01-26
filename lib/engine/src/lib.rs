#![feature(convert)]

extern crate rustc_serialize;

pub mod universe;
pub mod physics;
pub mod usage;
pub mod gamerules;
pub mod gamerules_test;
pub mod behavior;

#[test]
fn it_works() {
    let mut params = universe::DiskStorageEngineParams{
        dir: "/tmp/data/".to_string(),
    };

    let mut univ = universe::Universe::new(
        universe::Grid::new(
            universe::DiskStorageEngine::new(params)
        )
    );
   
    let mut rules: Box<gamerules::GameRules> = Box::new(gamerules_test::GameRulesTest::new());

    let mut phys = physics::StandardPhysics::create(univ, &mut rules).unwrap();

    match phys.run_tick(&mut rules) {
        Ok(_) => {}
        Err(e) => {panic!("err: {:?}", e);}
    }
}
