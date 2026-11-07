use crate::{app::App, event::handle_key_event};
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.get_state() {
        terminal.draw(|frame| draw(app, frame))?;
        handle_key_event(app)?;
    }
    Ok(())
}
pub fn draw(app: &mut App, frame: &mut Frame) {
    let title = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(app.get_title());
    frame.render_widget(title, frame.area());
}
