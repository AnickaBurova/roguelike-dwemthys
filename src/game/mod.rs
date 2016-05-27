use util::Bounds;
use rendering::RenderingComponent;

pub struct Game<'a> {
    exit :       bool,
    pub window_bounds : Bounds,
    pub rendering_component : Box<RenderingComponent + 'a>
}


impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let bound = Bounds::new(Point::new(0,0), Point::new(80,50));
        let mut root = Root::initializer()
            .size(80,50)
            .title("Dwemthys")
            .fullscreen(false)
            .init();
        let rc = Box::new(TcodRenderingComponent{root : root});
        Game{
            exit : false,
            window_bounds : bounds,
            rendering_component : rc
        }
    }

    pub fn render(&mut self, npcs : &Vec<Box<Updates>>, c : Character){
        self.rendering_component.pre_render();
        for i in npcs.iter() {
            i.render(&mut self.rendering_component());
        }
        c.render(&mut self.rendering_component());
        self.rendering_component.post_render();
    }

    pub fn update(&mut self, npcs :&mut Vec<Box<Updates>>, c : &mut Character, key : &KeyCode){
        c.update(key, self);
        for i in npcs.iter_mut(){
            i.update(self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rendering_component.wait_for_keypress()
    }

    pub fn 
}
