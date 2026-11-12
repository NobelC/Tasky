use crate::tui::drawing;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
};

pub fn draw_internal_section(frame: &mut Frame, available_area: &Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(*available_area);
    let right_panel = Block::default()
        .borders(Borders::LEFT | Borders::TOP)
        .border_type(BorderType::Rounded)
        .title("Task");
    let left_panel = Block::default()
        .borders(Borders::RIGHT | Borders::TOP)
        .border_type(BorderType::Rounded)
        .title(" Projects");
    frame.render_widget(left_panel, layout[0]);
    frame.render_widget(right_panel, layout[1]);
    drawing::draw_top_corner_left(frame, layout[0]);
    drawing::draw_bottom_corner_right(frame, layout[0]);
    drawing::draw_bottom_corner_left(frame, layout[1]);
    drawing::draw_top_corner_right(frame, layout[1]);
}
