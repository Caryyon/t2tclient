//TODO: connect to server with telnet
//TODO: figure out IO

use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use t2tclient::app::App;
use t2tclient::start_ui;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
