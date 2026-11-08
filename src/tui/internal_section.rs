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
        .borders(Borders::LEFT)
        .border_type(BorderType::Rounded);
    let left_panel = Block::default()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Rounded);

    let available_area_internal_left = left_panel.inner(layout[0]);

    frame.render_widget(left_panel, layout[0]);
    frame.render_widget(right_panel, layout[1]);
    draw_internal_section_left(frame, &available_area_internal_left);
}

pub fn draw_internal_section_left(frame: &mut Frame, available_area: &Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Fill(1)])
        .split(*available_area);
    let input_box = Block::bordered().border_type(BorderType::Rounded);
    let list_project = Block::default().border_type(BorderType::Rounded);
    frame.render_widget(input_box, layout[0]);
    frame.render_widget(list_project, layout[1]);
}
