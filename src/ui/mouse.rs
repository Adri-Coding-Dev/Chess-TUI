use crate::engine::board::Square;
use crate::ui::layout::BoardLayout;

pub fn pixel_to_square(col: u16, row: u16, board: &BoardLayout) -> Option<Square> {
    if col < board.rect.x || row < board.rect.y {
        return None;
    }
    let rel_x = col - board.rect.x;
    let rel_y = row - board.rect.y;
    if board.cell_width == 0 || board.cell_height == 0 {
        return None;
    }
    let file = rel_x / board.cell_width;
    let rank_from_bottom = rel_y / board.cell_height;
    if file >= 8 || rank_from_bottom >= 8 {
        return None;
    }
    let rank = 7 - rank_from_bottom;
    Some(rank as usize * 8 + file as usize)
}
