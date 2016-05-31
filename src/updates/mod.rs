// updates mod
extern crate tcod;

use tcod::input::{KeyCode};
use collision::{Aabb};

use game::Game;
use character::Character;
use npc::NPC;
use util::{Offset};
use rendering::RenderingComponent;


pub trait Updates{
    fn update(&mut self, KeyCode, &Game);
    fn render(&self, &mut RenderingComponent);
}

impl Updates for Character{
    fn update(&mut self, key : KeyCode, game : &Game){
        let mut offset = Offset::new(0,0);
        match key {
            KeyCode::Up         => offset.y = -1,
            KeyCode::Down       => offset.y = 1,
            KeyCode::Left       => offset.x = -1,
            KeyCode::Right      => offset.x  = 1,
            _                   => {}
        }

        let new_pos = self.position + offset;
        if game.window_bounds.contains(new_pos){
            self.position = new_pos;
        }
    }
    fn render(&self, rendering_component : &mut RenderingComponent){
        rendering_component.render(self.position, self.display_char);
    }
}

impl Updates for NPC{
    fn update(&mut self, key : KeyCode, game : &Game){
        self.position = self.mover.update(self.position,&game.window_bounds, key);
    }
    fn render(&self, rendering_component : &mut RenderingComponent){
        rendering_component.render(self.position, self.display_char);
    }
}
