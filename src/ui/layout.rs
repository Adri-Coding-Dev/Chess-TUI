use ratatui::layout::{Rect, Constraint, Direction, Layout};
use super::theme::Theme;

pub struct AppChunks {
    pub left: Rect,
    pub board: Rect,
    pub right: Rect,
}

pub fn create_layout(area: Rect, _theme: &Theme) -> AppChunks {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    // Calculate board square size
    let board_area = horizontal[1];
    let board_width = board_area.width.min(board_area.height); // keep square
    let board_height = board_width; // same as width
    let board_x = board_area.x + (board_area.width - board_width) / 2;
    let board_y = board_area.y + (board_area.height - board_height) / 2;
    let board_rect = Rect::new(board_x, board_y, board_width, board_height);

    AppChunks {
        left: horizontal[0],
        board: board_rect,
        right: horizontal[2],
    }
}
