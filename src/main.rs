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
