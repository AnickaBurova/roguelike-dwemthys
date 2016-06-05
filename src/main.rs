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
mod world;
mod component;


use self::game::Game;
use self::world::World;
use self::rendering::{RenderingComponent};

//use tcod::input::{KeyCode,KeyPressFlags};



fn main() {

    let mut game = Game::new();
    let mut world = World::new(&game);

    game.render(&world);
    while !game.finished() {
        /*let key = root.check_for_keypress(KeyPressFlags::empty());
        let code = match key {
            Some(k) => k.code,
            None => KeyCode::NoKey
        };*/
        game.update(&mut world);

        game.render(&world);
    }
}
