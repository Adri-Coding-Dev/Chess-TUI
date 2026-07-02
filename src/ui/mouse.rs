use ratatui::layout::Rect;
use crate::engine::board::Square;

pub fn pixel_to_square(col: u16, row: u16, board_area: &Rect) -> Option<Square> {
    if col < board_area.x || row < board_area.y {
        return None;
    }
    let rel_x = col - board_area.x;
    let rel_y = row - board_area.y;
    let cell_width = board_area.width / 8;
    let cell_height = board_area.height / 8;
    if cell_width == 0 || cell_height == 0 {
        return None;
    }
    let file = rel_x / cell_width;
    let rank_from_bottom = rel_y / cell_height;
    if file >= 8 || rank_from_bottom >= 8 {
        return None;
    }
    // Board: rank 0 = bottom
    let rank = 7 - rank_from_bottom;
    Some(rank as usize * 8 + file as usize)
}
