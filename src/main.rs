extern crate rand;
extern crate cgmath;
extern crate collision;
extern crate tcod;

mod game;
mod util;
mod updates;
mod character;
mod npc;
mod rendering;
mod input;


use self::game::Game;
use self::updates::Updates;
use self::character::Character;
use self::npc::NPC;
use self::rendering::{RenderingComponent};

//use tcod::input::{KeyCode,KeyPressFlags};


#[test]
fn test_collision(){
    let p = Point::new(-20isize,30isize);
    let aabb = Aabb2::new(Point::new(-20,30), Point::new(10,-10));
    assert!(aabb.contains(Point::new(0,0)));
}



fn main() {

    let mut game = Game::new();

    let mut c = Character::new(40,25,'@');
    let mut npcs : Vec<Box<Updates>> = vec![];
    for _ in 0..10{
        let d = Box::new(NPC::new_in_game(&game,'d')) as Box<Updates>;
        npcs.push(d);
    }
    game.render(&npcs, &c);
    while !game.finished() {
        /*let key = root.check_for_keypress(KeyPressFlags::empty());
        let code = match key {
            Some(k) => k.code,
            None => KeyCode::NoKey
        };*/
        game.update(&mut npcs, &mut c);

        game.render(&npcs, &c);
    }
}
