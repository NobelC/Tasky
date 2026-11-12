use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

pub fn calculate_area_available(frame: &mut Frame, block: Block) -> Rect {
    block.inner(frame.area())
}

pub fn draw_top_corner_right(frame: &mut Frame, area: Rect) {
    let top_corner_right = Paragraph::new("╮");
    frame.render_widget(
        top_corner_right,
        Rect::new(area.x + area.width.saturating_sub(1), area.y, 1, 1),
    );
}
pub fn draw_top_corner_left(frame: &mut Frame, area: Rect) {
    let top_corner_left = Paragraph::new("╭");
    frame.render_widget(top_corner_left, Rect::new(area.x, area.y, 1, 1));
}
pub fn draw_bottom_corner_left(frame: &mut Frame, area: Rect) {
    let bottom_corner_left = Paragraph::new("╰");
    frame.render_widget(
        bottom_corner_left,
        Rect::new(area.x, area.y + area.height.saturating_sub(1), 1, 1),
    );
}
pub fn draw_bottom_corner_right(frame: &mut Frame, area: Rect) {
    let bottom_corner_left = Paragraph::new("╯");
    frame.render_widget(
        bottom_corner_left,
        Rect::new(
            area.x + area.width.saturating_sub(1),
            area.y + area.height.saturating_sub(1),
            1,
            1,
        ),
    );
}
