
use util::Point;

pub struct Character{
    pub position :      Point,
    pub display_char :  char
}

impl Character{
    pub fn new(x : i32, y : i32, dc : char) -> Character{
        Character{position : Point::new(x,y), display_char : dc}
    }
}
