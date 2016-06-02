// GAME
extern crate rand;

use util::{Point,Bounds};
use rendering::{RenderingComponent, TcodRenderingComponent};
use actor::Actor;
use input::{RandomMovementComponent,MovementComponent};
use tcod_impl::TcodInputMovementComponent;


use tcod::console::Root;
use tcod::input::{KeyCode,Key};
use std::rc::Rc;
use rand::Rng;

pub struct Game<'a> {
    exit :       bool,
    pub window_bounds : Bounds,
    rendering_component : Box<RenderingComponent + 'a>,
    actors : Vec<Box<Actor>>,
}


impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let bounds = Bounds::new(Point::new(0,0), Point::new(80,50));
        let root = Root::initializer()
            .size(80,50)
            .title("Dwemthys")
            .fullscreen(false)
            .init();
        let rc = Box::new(TcodRenderingComponent::new(root));
        Game{
            exit : false,
            window_bounds : bounds,
            rendering_component : rc,
            actors : vec![],
        }
    }

    pub fn render(&mut self){
        self.rendering_component.pre_render();
        for i in self.actors.iter() {
            i.render(&mut *self.rendering_component);
        }
        self.rendering_component.post_render();
    }

    pub fn update(&mut self){
        let key = self.wait_for_keypress();
        let code = key.code;
        match code{
            KeyCode::Escape => self.exit = true,
            _ => {}
        }
        // update will be done in 2 stages
        // 1st: all actors will move in to new position and fire
        // 2nd: all actors will accumulate damage if any and potentially die
        let mut actors = Vec::with_capacity(self.actors.len());
        for a in self.actors.iter(){
            actors.push(Box::new(a.update(code,&self)));
        }
        self.actors = actors;
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rendering_component.wait_for_keypress()
    }

    pub fn finished(&self) -> bool {
        self.rendering_component.window_closed() || self.exit
    }

    pub fn create_level(&mut self) {
        self.actors = vec![];
        // npcs
        let rand_mover = Rc::new(RandomMovementComponent::new()) as Rc<MovementComponent>;
        let npcs = ['d', 'c', 'k'];
        for _ in 0..10{
            let npc = rand::thread_rng().gen_range(0,npcs.len());
            let d = Box::new(Actor::new_in_game(&self,npcs[npc],rand_mover.clone()));
            self.actors.push(d);
        }
        // create heroine
        let input_mover = Rc::new(TcodInputMovementComponent::new()) as Rc<MovementComponent>;
        let h = Box::new(Actor::new(40,25,'@',input_mover.clone()));
        self.actors.push(h);
    }
}
