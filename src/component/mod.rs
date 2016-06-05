//! Component
//!
//!
use game::Game;
use world::World;
use actor::Actor;

/// Updates  
pub trait Component{
    fn update(&mut self, &mut Actor, &Game, &mut World);
}
