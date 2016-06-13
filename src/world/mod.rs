//! World
extern crate rand;

use actor::Actor;
use game::Game;
use input::{RandomMovementComponent,MovementComponent};
use tcod_impl::TcodInputMovementComponent;

use std::rc::Rc;
use rand::Rng;
use std::cell::RefCell;

pub struct World{
    pub actors : Vec<Rc<RefCell<Actor>>>,
}

impl World{
    pub fn new(game : &Game) -> World{
        let mut actors = vec![];
        // npcs
        let rand_mover = Rc::new(RandomMovementComponent::new()) as Rc<MovementComponent>;
        let npcs = ['d', 'c', 'k'];
        for _ in 0..10{
            let npc = rand::thread_rng().gen_range(0,npcs.len());
            let d = Rc::new(RefCell::new(Actor::new_in_game(&game,npcs[npc],rand_mover.clone())));
            actors.push(d);
        }
        // create heroine
        let input_mover = Rc::new(TcodInputMovementComponent::new()) as Rc<MovementComponent>;
        let h = Rc::new(RefCell::new(Actor::new(40,25,'@',input_mover.clone())));
        actors.push(h);
        World{
            actors : actors,
        }
    }
}
