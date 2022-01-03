use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;

use app::{App, AppReturn};
use eyre::Result;
use inputs::events::Events;
use inputs::InputEvent;
use tui::backend::TermionBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;
pub mod inputs;

#[allow(unreachable_code)]
pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;


    // User event handler
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);


    loop {
        let app = app.borrow();

        // Render
        terminal.draw(|frame| ui::draw(frame, &app))?;

         // Handle inputs
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
