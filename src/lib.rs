use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    //loop {
    // setup TUI layout
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ].as_ref()
            )
            .split(frame.size());
        // Main area block
        let user_feed = Block::default()
             .title("Body")
             .borders(Borders::ALL);
        frame.render_widget(user_feed, chunks[0]);
        // Input area Block
        let user_input = Block::default()
             .title("Input")
             .borders(Borders::ALL);
        frame.render_widget(user_input, chunks[1]);
    })?;
    //};

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
