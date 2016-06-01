// updates mod

use tcod::input::{KeyCode};

use game::Game;
use actor::Actor;
use rendering::RenderingComponent;


pub trait Updates{
    fn update(&mut self, KeyCode, &Game);
    fn render(&self, &mut RenderingComponent);
}


impl Updates for Actor{
    fn update(&mut self, key : KeyCode, game : &Game){
        self.position = self.mover.update(self.position,&game.window_bounds, key);
    }
    fn render(&self, rendering_component : &mut RenderingComponent){
        rendering_component.render(self.position, self.display_char);
    }
}
