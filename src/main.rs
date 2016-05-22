extern crate tcod;

use tcod::console::Root;
use tcod::{Console,BackgroundFlag};
use tcod::input::{KeyCode};


fn render(con : &mut Root) {
    con.clear();
    con.put_char(40,25, '@', BackgroundFlag::Set);
    con.flush();
}

fn main() {
    let mut root = Root::initializer()
        .size(80,50)
        .title("Dwemthys")
        .fullscreen(false)
        .init();
    let mut exit = false;
    render(&mut root);
    while !(root.window_closed() || exit){
        let keypress = root.wait_for_keypress(true);
        match keypress.code{
            KeyCode::Escape => exit = true,
            _ => {}
        }
        render(&mut root);
    }
}
