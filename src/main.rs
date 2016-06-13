//! =====================================================================================
//!
//!       Filename:  main.rs
//!
//!    Description:  The main app file.
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
extern crate cgmath;
extern crate collision;
extern crate tcod;

mod game;
mod util;
mod actor;
mod rendering;
mod world;
mod component;
mod movement;


use self::game::Game;
use self::world::World;
use self::rendering::{RenderingComponent};



fn main() {

    let mut game = Game::new();
    let mut world = World::new(&game);

    game.render(&world);
    while !game.finished() {
        game.update(&mut world);

        game.render(&world);
    }
}
