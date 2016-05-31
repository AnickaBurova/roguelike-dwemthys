// tcod impl
use input::MovementComponent;
use util::{Point,Offset,Bounds};

use tcod::input::KeyCode;
use collision::Aabb;



pub struct TcodInputMovementComponent;

impl TcodInputMovementComponent{
    pub fn new() -> Self{
        TcodInputMovementComponent{}
    }
}

impl MovementComponent for TcodInputMovementComponent {
    fn update(&self, origin : Point, bounds : &Bounds, key : KeyCode) -> Point {
        let mut offset = Offset::new(0,0);
        match key {
            KeyCode::Up         => offset.y = -1,
            KeyCode::Down       => offset.y = 1,
            KeyCode::Left       => offset.x = -1,
            KeyCode::Right      => offset.x  = 1,
            _                   => {}
        }

        let new_pos = origin + offset;
        if bounds.contains(new_pos){
            new_pos
        }
        else{
            origin
        }
    }
}
