// input
extern crate rand;

use rand::Rng;
use tcod::input::{KeyCode};
use collision::{Aabb};

use util::{Point,Bounds,Offset};

pub trait MovementComponent {
    fn update(&self, Point, &Bounds, KeyCode) -> Point;
}

pub struct RandomMovementComponent;

impl RandomMovementComponent {
    pub fn new() -> RandomMovementComponent {
        RandomMovementComponent{}
    }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, origin : Point, bounds : &Bounds, _ : KeyCode) -> Point {
        let offset_x = rand::thread_rng().gen_range(-1,2);
        let new_pos = origin + Offset::new(offset_x, 0);
        let mut res = origin;
        if bounds.contains(new_pos){
            res = new_pos;
        }
        let offset_y = rand::thread_rng().gen_range(-1,2);
        let new_pos = res + Offset::new(0, offset_y);
        if bounds.contains(new_pos){
            res = new_pos;
        }
        res
    }
}


