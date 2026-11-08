use crate::{app::App, event::handle_key_event, utilities::calculate_area_available};
use ratatui::widgets::{Block, BorderType};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub mod internal_section;
use internal_section::draw_internal_section;

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
    let available_area_internal_section = calculate_area_available(frame, title.clone());
    frame.render_widget(title, frame.area());
    draw_internal_section(frame, &available_area_internal_section);
}
