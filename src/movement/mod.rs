extern crate rand;

use rand::Rng;
use collision::{Aabb};
use tcod::input::KeyCode;

use component::Component;
use util::{Point,Bounds,Offset};
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