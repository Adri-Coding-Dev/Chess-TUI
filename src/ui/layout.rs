use ratatui::layout::{Rect, Constraint, Direction, Layout};
use super::theme::Theme;

#[derive(Clone, Copy)]
pub struct BoardLayout {
    pub rect: Rect,
    pub cell_width: u16,
    pub cell_height: u16,
}

pub struct AppChunks {
    pub left: Rect,
    pub board: BoardLayout,
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

    let board_area = horizontal[1];
    let max_cell_w = board_area.width / 8;
    if max_cell_w == 0 {
        return AppChunks {
            left: horizontal[0],
            board: BoardLayout { rect: board_area, cell_width: 1, cell_height: 1 },
            right: horizontal[2],
        };
    }

    // desired cell height based on width: (w + 1) / 2 to make it visually square
    let ideal_cell_h = (max_cell_w + 1) / 2;
    let mut cell_w = max_cell_w;
    let mut cell_h = ideal_cell_h;
    if cell_h * 8 > board_area.height {
        // need to reduce cell_w so that cell_h fits
        let max_cell_h = board_area.height / 8;
        if max_cell_h == 0 {
            return AppChunks {
                left: horizontal[0],
                board: BoardLayout { rect: board_area, cell_width: 1, cell_height: 1 },
                right: horizontal[2],
            };
        }
        cell_w = (max_cell_h * 2).saturating_sub(1);
        cell_w = cell_w.min(max_cell_w);
        cell_h = (cell_w + 1) / 2;
    }

    let board_pixel_w = cell_w * 8;
    let board_pixel_h = cell_h * 8;
    let board_x = board_area.x + (board_area.width - board_pixel_w) / 2;
    let board_y = board_area.y + (board_area.height - board_pixel_h) / 2;
    let board_rect = Rect::new(board_x, board_y, board_pixel_w, board_pixel_h);

    AppChunks {
        left: horizontal[0],
        board: BoardLayout {
            rect: board_rect,
            cell_width: cell_w,
            cell_height: cell_h,
        },
        right: horizontal[2],
    }
}
