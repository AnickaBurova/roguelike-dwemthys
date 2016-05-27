extern crate rand;
extern crate tcod;
extern crate cgmath;
extern crate collision;

mod game;
mod util;
mod updates;
mod character;
mod npc;
mod rendering;


use self::game::Game;
use self::util::{Point, Bounds};
use self::updates::Updates;
use self::character::Character;
use self::npc::NPC;
use self::rendering::{RenderingComponent,TcodRenderingComponent};

use tcod::console::Root;
use tcod::{Console};
use tcod::input::{KeyCode};
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
    let mut rendering_component= Box::new( TcodRenderingComponent{root: root});
    render(&mut rendering_component, &npcs, &c);
    while !(rendering_component.window_closed() || game.exit){
        /*let key = root.check_for_keypress(KeyPressFlags::empty());
        let code = match key {
            Some(k) => k.code,
            None => KeyCode::NoKey
        };*/
        let key = rendering_component.wait_for_keypress();
        let code = key.code;
        match code{
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        update(&mut npcs, &mut c, &code, &game);
        render(&mut rendering_component, &npcs, &c);
    }
}
