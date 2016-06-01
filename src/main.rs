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
use self::rendering::{RenderingComponent};

//use tcod::input::{KeyCode,KeyPressFlags};



fn main() {

    let mut game = Game::new();
    game.create_level();

    game.render();
    while !game.finished() {
        /*let key = root.check_for_keypress(KeyPressFlags::empty());
        let code = match key {
            Some(k) => k.code,
            None => KeyCode::NoKey
        };*/
        game.update();

        game.render();
    }
}
