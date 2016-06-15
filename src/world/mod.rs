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
use movement::{RandomMovement,InputMovement,ChaseMovement};
use component::Component;

use std::rc::Rc;
use rand::Rng;
use std::cell::{RefCell,BorrowState};

pub struct World{
    pub actors : Rc<Vec<Rc<RefCell<Actor>>>>,
}

impl World{
    pub fn new(game : &Game) -> World{
        let mut actors = vec![];
        // npcs
        let rand_mover = Rc::new(RefCell::new(RandomMovement{})) as Rc<RefCell<Component>>;
        let chase_mover = Rc::new(RefCell::new(ChaseMovement{target: '@'})) as Rc<RefCell<Component>>;
        let npcs = ['d', 'c', 'k'];
        let movers = [rand_mover.clone(), chase_mover.clone(), rand_mover.clone()];
        for _ in 0..10 {
            let npc = rand::thread_rng().gen_range(0,npcs.len());
            let d = Rc::new(RefCell::new(Actor::new_in_game(&game,npcs[npc])));
            d.borrow_mut().add_component(movers[npc].clone());
            actors.push(d);
        }
        // create heroine
        let input_mover = Rc::new(RefCell::new(InputMovement{})) as Rc<RefCell<Component>>;
        let h = Rc::new(RefCell::new(Actor::new(40,25,'@')));
        h.borrow_mut().add_component(input_mover.clone());
        actors.push(h);
        World{
            actors : Rc::new(actors),
        }
    }
    pub fn find(&self, target: char) -> Option<Rc<RefCell<Actor>>> {
        for actor in self.actors.iter() {
            if actor.borrow_state() != BorrowState::Writing && actor.borrow().display_char == target {
                return Some(actor.clone());
            }
        }
        None
    }
}


#[test]
fn test_find() {
    let mut actors = vec![];
    let h = Rc::new(RefCell::new(Actor::new(40,25,'@')));
    actors.push(h);
    let world = World{actors : Rc::new(actors)};
    let found = world.find('@');
    assert!(found.is_some());
    let found = world.find('x');
    assert!(found.is_none());
    //assert_eq!(h.borrow(),found.borrow());
}
