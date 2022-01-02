
// This is a terminal user interface(TUI) client
// for the http://t2tmud.org MUD(Multi-User Dungeon) Game.

// This client aims to wrap around the direct game IO and add some unique
// functionality not offered by the original game creators.
// - map wrapper (moves map command to upper right panel)
// - inventory wrapper (see what you got on)
// - hand wrapper (see what you got in your hands)
// - and more!!!

//TODO: connect to server with telnet
//TODO: figure out IO

use std::io;

fn main() -> Result<(), io::Error> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}
