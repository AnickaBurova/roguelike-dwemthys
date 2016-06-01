extern crate rand;
extern crate cgmath;
extern crate collision;
extern crate tcod;

mod game;
mod util;
mod actor;
mod rendering;
mod input;
mod tcod_impl;


use self::game::Game;
use self::actor::Actor;
use self::rendering::{RenderingComponent};
use self::input::{RandomMovementComponent,MovementComponent};
use self::tcod_impl::TcodInputMovementComponent;

//use tcod::input::{KeyCode,KeyPressFlags};
use std::rc::Rc;



fn main() {

    let mut game = Game::new();

    let input_mover = Rc::new(TcodInputMovementComponent::new()) as Rc<MovementComponent>;
    let mut c = Actor::new(40,25,'@',input_mover.clone());
    let mut actors : Vec<Box<Actor>> = vec![];
    let rand_mover = Rc::new(RandomMovementComponent::new()) as Rc<MovementComponent>;
    for _ in 0..10{
        let d = Box::new(Actor::new_in_game(&game,'d',rand_mover.clone()));
        actors.push(d);
    }
    game.render(&actors, &c);
    while !game.finished() {
        /*let key = root.check_for_keypress(KeyPressFlags::empty());
        let code = match key {
            Some(k) => k.code,
            None => KeyCode::NoKey
        };*/
        game.update(&mut actors, &mut c);

        game.render(&actors, &c);
    }
}
