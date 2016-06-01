extern crate rand;
extern crate cgmath;
extern crate collision;
extern crate tcod;

mod game;
mod util;
mod updates;
mod actor;
mod rendering;
mod input;
mod tcod_impl;


use self::game::Game;
use self::updates::Updates;
use self::actor::Actor;
use self::rendering::{RenderingComponent};
use self::input::{RandomMovementComponent,MovementComponent};
use self::tcod_impl::TcodInputMovementComponent;

//use tcod::input::{KeyCode,KeyPressFlags};
use std::rc::Rc;


#[test]
fn test_collision(){
    let p = Point::new(-20isize,30isize);
    let aabb = Aabb2::new(Point::new(-20,30), Point::new(10,-10));
    assert!(aabb.contains(Point::new(0,0)));
}



fn main() {

    let mut game = Game::new();

    let input_mover = Rc::new(TcodInputMovementComponent::new()) as Rc<MovementComponent>;
    let mut c = Actor::new(40,25,'@',input_mover.clone());
    let mut actors : Vec<Box<Updates>> = vec![];
    let rand_mover = Rc::new(RandomMovementComponent::new()) as Rc<MovementComponent>;
    for _ in 0..10{
        let d = Box::new(Actor::new_in_game(&game,'d',rand_mover.clone())) as Box<Updates>;
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
