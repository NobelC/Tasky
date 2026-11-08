use ratatui::{Frame, layout::Rect, widgets::Block};

pub fn calculate_area_available(frame: &mut Frame, block: Block) -> Rect {
    block.inner(frame.area())
}
