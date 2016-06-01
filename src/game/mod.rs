// GAME
use util::{Point,Bounds};
use rendering::{RenderingComponent, TcodRenderingComponent};
use updates::Updates;


use tcod::console::Root;
use tcod::input::{KeyCode,Key};

pub struct Game<'a> {
    exit :       bool,
    pub window_bounds : Bounds,
    rendering_component : Box<RenderingComponent + 'a>
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
            rendering_component : rc
        }
    }

    pub fn render(&mut self, actors : &Vec<Box<Updates>>, c : &Updates){
        self.rendering_component.pre_render();
        for i in actors.iter() {
            i.render(&mut *self.rendering_component);
        }
        c.render(&mut *self.rendering_component);
        self.rendering_component.post_render();
    }

    pub fn update(&mut self, actors :&mut Vec<Box<Updates>>, c : &mut Updates){
        let key = self.wait_for_keypress();
        let code = key.code;
        match code{
            KeyCode::Escape => self.exit = true,
            _ => {}
        }

        c.update(code, self);
        for i in actors.iter_mut(){
            i.update(code,self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rendering_component.wait_for_keypress()
    }

    pub fn finished(&self) -> bool {
        self.rendering_component.window_closed() || self.exit
    }
}
