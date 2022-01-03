use tui::backend::Backend;
use tui::layout::{ Constraint, Direction, Layout, Rect};
//use tui::style::{Color, Style};
use tui::widgets::{Block,  Borders};
use tui::Frame;

use crate::app::App;

pub fn draw<B>(frame: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let size = frame.size();
    check_size(&size);

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
}


fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}
