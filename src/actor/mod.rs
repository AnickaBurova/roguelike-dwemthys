//! =====================================================================================
//!
//!       Filename:  actor/mod.rs
//!
//!    Description:  Actor in the game.
//!
//!        Version:  1.0
//!        Created:  13/06/16 22:43:05
//!       Revision:  none
//!       Compiler:  rust
//!
//!         Author:  Anicka Burova
//!
//! =====================================================================================

extern crate rand;

use util::{Point};
use game::Game;
use collision::{Aabb};
use rendering::RenderingComponent;
use world::World;
use component::Component;

use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Actor{
    pub position :      Point,
    pub display_char :  char,
    pub components :    Rc<Vec<Rc<RefCell<Component>>>>,
}

impl Actor{
    pub fn new(x : i32, y : i32, dc : char) -> Actor{
        Actor{position : Point::new(x,y), display_char : dc, components : Rc::new(vec![])}
    }
    pub fn new_in_game(game : &Game,dc : char) -> Actor{
        Actor{position : Point::new(
                rand::thread_rng().gen_range(game.window_bounds.min().x, game.window_bounds.max().x),
                rand::thread_rng().gen_range(game.window_bounds.min().y, game.window_bounds.max().y)),
                display_char : dc,
                components : Rc::new(vec![]),
        }
    }
    pub fn update(&mut self, game : &mut Game, world : &mut World) {
        let components = self.components.clone();
        for c in components.iter(){
            c.borrow_mut().update(self,game,world);
        }
    }
    pub fn render(&self, rendering_component : &mut RenderingComponent){
        rendering_component.render(self.position, self.display_char);
    }

    pub fn add_component( &mut self, cmp: Rc<RefCell<Component>>) {
        Rc::make_mut(&mut self.components).push(cmp);
    }
}
