//! =====================================================================================
//!
//!       Filename:  movement/mod.rs
//!
//!    Description:  Components to update actor's movement.
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

use rand::Rng;
use collision::{Aabb};
use tcod::input::KeyCode;

use component::Component;
use util::{Offset};
use actor::Actor;
use game::Game;
use world::World;

pub struct RandomMovement;


impl Component for RandomMovement {
    fn update(&mut self, actor: &mut Actor, game: &Game, _: &mut World) {
        let offset_x = rand::thread_rng().gen_range(-1,2);
        let new_pos = actor.position + Offset::new(offset_x, 0);
        let mut res = actor.position;
        if game.window_bounds.contains(new_pos) {
            res = new_pos;
        }
        let offset_y = rand::thread_rng().gen_range(-1,2);
        let new_pos = res + Offset::new(0, offset_y);
        if game.window_bounds.contains(new_pos) {
            res = new_pos;
        }
        actor.position = res;
    }
}

pub struct InputMovement;

impl Component for InputMovement {
    fn update(&mut self, actor: &mut Actor, game: &Game, _: &mut World) {
        let mut offset = Offset::new(0,0);
        let key = game.last_key;
        match key {
            KeyCode::Up         => offset.y = -1,
            KeyCode::Down       => offset.y = 1,
            KeyCode::Left       => offset.x = -1,
            KeyCode::Right      => offset.x  = 1,
            _                   => {}
        }

        let new_pos = actor.position + offset;
        if game.window_bounds.contains(new_pos) {
            actor.position = new_pos;
        }
    }
}

pub struct ChaseMovement {
    pub target: char,
}

impl Component for ChaseMovement {
    fn update(&mut self, actor: &mut Actor, _: &Game, world: &mut World) {
        // find character to chase
        let target = world.find(self.target);
        match target {
            Some(ref t) => {
                // start moving toward the target
                let target = t.borrow();
                let x_delta = target.position.x - actor.position.x;
                let y_delta = target.position.y - actor.position.y;
                let mut offset = Offset::new(0,0);
                if x_delta.abs() > y_delta.abs() {
                    offset.x = x_delta.signum();
                } else {
                    offset.y = y_delta.signum();
                }
                actor.position = actor.position + offset;
            }
            _ => {},// do nothing if we dont have a target
        }
    }
}
