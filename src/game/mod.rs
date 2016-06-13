// GAME
extern crate rand;

use util::{Point,Bounds};
use rendering::{RenderingComponent, TcodRenderingComponent};
use world::World;


use tcod::console::Root;
use tcod::input::{KeyCode,Key};

pub struct Game {
    exit :       bool,
    pub window_bounds : Bounds,
    rendering_component : Box<RenderingComponent>,
    pub last_key: KeyCode,
}


impl Game {
    pub fn new() -> Game {
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
            last_key : KeyCode::Escape,
        }
    }

    pub fn render(&mut self, world : &World){
        self.rendering_component.pre_render();
        for i in world.actors.iter() {
            i.borrow().render(&mut *self.rendering_component);
        }
        self.rendering_component.post_render();
    }

    pub fn update(&mut self, world : &mut World){
        let key = self.wait_for_keypress();
        self.last_key = key.code;
        match self.last_key{
            KeyCode::Escape => self.exit = true,
            _ => {}
        }

        let actors = world.actors.clone();

        for a in actors.iter(){
            a.borrow_mut().update(self,world);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rendering_component.wait_for_keypress()
    }

    pub fn finished(&self) -> bool {
        self.rendering_component.window_closed() || self.exit
    }

}
