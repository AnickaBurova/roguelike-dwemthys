//! =====================================================================================
//!
//!       Filename:  world/mod.rs
//!
//!    Description:  The world, it contains actors.
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

use actor::Actor;
use game::Game;
use movement::{RandomMovement,InputMovement};
use component::Component;

use std::rc::Rc;
use rand::Rng;
use std::cell::RefCell;

pub struct World{
    pub actors : Rc<Vec<Rc<RefCell<Actor>>>>,
}

impl World{
    pub fn new(game : &Game) -> World{
        let mut actors = vec![];
        // npcs
        let rand_mover_cmp = Rc::new(RefCell::new(RandomMovement{})) as Rc<RefCell<Component>>;
        let npcs = ['d', 'c', 'k'];
        for _ in 0..10{
            let npc = rand::thread_rng().gen_range(0,npcs.len());
            let d = Rc::new(RefCell::new(Actor::new_in_game(&game,npcs[npc])));
            d.borrow_mut().add_component(rand_mover_cmp.clone());
            actors.push(d);
        }
        // create heroine
        let input_mover_cmp = Rc::new(RefCell::new(InputMovement{})) as Rc<RefCell<Component>>;
        let h = Rc::new(RefCell::new(Actor::new(40,25,'@')));
        h.borrow_mut().add_component(input_mover_cmp.clone());
        actors.push(h);
        World{
            actors : Rc::new(actors),
        }
    }
}
