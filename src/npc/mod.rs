// NPC mod
extern crate rand;
extern crate collision;

use util::{Point};
use game::Game;
use collision::{Aabb};
use input::MovementComponent;

use rand::Rng;
use std::rc::Rc;

pub struct NPC{
    pub position :      Point,
    pub display_char :  char,
    pub mover : Rc<MovementComponent>,
}

impl NPC{
    //pub fn new(x : i32, y : i32, dc : char) -> NPC{
        //NPC{position : Point::new(x,y), display_char : dc}
    //}
    pub fn new_in_game(game : &Game,dc : char, mover : Rc<MovementComponent>) -> NPC{
        NPC{position : Point::new(
                rand::thread_rng().gen_range(game.window_bounds.min().x, game.window_bounds.max().x)
                ,rand::thread_rng().gen_range(game.window_bounds.min().y, game.window_bounds.max().y))
                ,display_char : dc
                ,mover : mover
        }
    }
}
