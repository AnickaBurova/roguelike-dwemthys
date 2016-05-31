// rendering
use tcod::console::Root;
use tcod::{Console,BackgroundFlag};
use tcod::input::Key;

use util::Point;

pub trait RenderingComponent{
    fn pre_render(&mut self);
    fn render(&mut self, Point, char);
    fn post_render(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn window_closed(&self) -> bool;
}

pub struct TcodRenderingComponent {
    root : Root
}

impl TcodRenderingComponent{
    pub fn new(root : Root) -> TcodRenderingComponent{
        TcodRenderingComponent{root : root}
    }
}

impl RenderingComponent for TcodRenderingComponent{
    
    fn pre_render(&mut self){
        self.root.clear();
    }

    fn render(&mut self, position : Point, symbol : char) {
        self.root.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    fn post_render(&mut self) {
        self.root.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }

    fn window_closed(&self) -> bool {
        self.root.window_closed()
    }
}
