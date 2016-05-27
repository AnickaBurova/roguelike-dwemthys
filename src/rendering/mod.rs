extern crate  tcod;

use tcod::console::Root;
use tcod::{Console,BackgroundFlag};
use tcod::input::Key;

use util::Point;

pub trait RenderingComponent{
    fn pre_render(&mut self);
    fn render(&mut self, Point, char);
    fn post_render(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn window_closed() -> bool;
}

pub struct TcodRenderingComponent {
    root : Root
}

impl RenderingComponent for TcodRenderingComponent{
    pub fn pre_render(&mut self){
        self.root.clear();
    }

    pub fn render(&mut self, position : Point, symbol : char) {
        self.root.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    pub fn post_render(&mut self) {
        self.root.flush();
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }

    pub fn window_closed(&mut self) -> bool {
        self.root.window_closed()
    }
}
