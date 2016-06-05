// Actor mod
extern crate rand;

use util::{Point};
use game::Game;
use collision::{Aabb};
use input::MovementComponent;
use rendering::RenderingComponent;
use world::World;

use tcod::input::{KeyCode};
use rand::Rng;
use std::rc::Rc;

pub struct Actor{
    pub position :      Point,
    pub display_char :  char,
    pub mover : Rc<MovementComponent>,
}

impl Actor{
    pub fn new(x : i32, y : i32, dc : char, mover : Rc<MovementComponent>) -> Actor{
        Actor{position : Point::new(x,y), display_char : dc, mover : mover}
    }
    pub fn new_in_game(game : &Game,dc : char, mover : Rc<MovementComponent>) -> Actor{
        Actor{position : Point::new(
                rand::thread_rng().gen_range(game.window_bounds.min().x, game.window_bounds.max().x)
                ,rand::thread_rng().gen_range(game.window_bounds.min().y, game.window_bounds.max().y))
                ,display_char : dc
                ,mover : mover
        }
    }
    pub fn update(&mut self, key : KeyCode, game : &mut Game, world : &mut World) {
        let position = self.mover.update(self.position,&game.window_bounds, key);
        self.position = position;
    }
    pub fn render(&self, rendering_component : &mut RenderingComponent){
        rendering_component.render(self.position, self.display_char);
    }
}
