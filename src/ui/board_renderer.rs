use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Span, Line},
    widgets::Paragraph,
};
use crate::engine::game::Game;
use crate::engine::board::{Piece, Color as PieceColor};
use crate::states::UiState;
use super::theme::Theme;
use super::layout::BoardLayout;

pub fn render_board(frame: &mut Frame, board_layout: &BoardLayout, game: &Game, ui_state: &UiState, theme: &Theme) {
    let BoardLayout { rect, cell_width, cell_height } = *board_layout;
    if cell_width == 0 || cell_height == 0 {
        return;
    }

    for rank in 0..8 {
        for file in 0..8 {
            let sq = (7 - rank) * 8 + file;
            let piece_info = game.board().piece_at(sq);
            let is_light = (rank + file) % 2 == 0;

            let mut bg = if is_light { theme.light_square } else { theme.dark_square };

            if let UiState::PieceSelected { from, .. } = ui_state {
                if sq == *from {
                    bg = theme.selection;
                }
            }
            if let UiState::PieceSelected { legal_moves, .. } = ui_state {
                if legal_moves.iter().any(|m| m.to == sq) {
                    bg = theme.move_hint;
                }
            }

            let piece_str = piece_info.map(|(c, p)| piece_to_unicode(c, p)).unwrap_or(' ');
            let cell_x = rect.x + file as u16 * cell_width;
            let cell_y = rect.y + rank as u16 * cell_height;

            let empty_line = " ".repeat(cell_width as usize);
            let piece_line = if cell_width > 0 {
                let padding = (cell_width as usize - 1) / 2;
                format!(
                    "{:>pad$}{}{:pad$}",
                    "", piece_str, "",
                    pad = padding
                )
            } else {
                piece_str.to_string()
            };

            let lines: Vec<Line> = (0..cell_height).map(|i| {
                let content = if i == cell_height / 2 {
                    piece_line.clone()
                } else {
                    empty_line.clone()
                };
                Line::from(Span::styled(content, Style::default().fg(theme.text).bg(bg)))
            }).collect();

            let cell_area = Rect {
                x: cell_x,
                y: cell_y,
                width: cell_width,
                height: cell_height,
            };
            frame.render_widget(Paragraph::new(lines), cell_area);
        }
    }
}

fn piece_to_unicode(color: PieceColor, piece: Piece) -> char {
    match (color, piece) {
        (PieceColor::White, Piece::King) => '♔',
        (PieceColor::White, Piece::Queen) => '♕',
        (PieceColor::White, Piece::Rook) => '♖',
        (PieceColor::White, Piece::Bishop) => '♗',
        (PieceColor::White, Piece::Knight) => '♘',
        (PieceColor::White, Piece::Pawn) => '♙',
        (PieceColor::Black, Piece::King) => '♚',
        (PieceColor::Black, Piece::Queen) => '♛',
        (PieceColor::Black, Piece::Rook) => '♜',
        (PieceColor::Black, Piece::Bishop) => '♝',
        (PieceColor::Black, Piece::Knight) => '♞',
        (PieceColor::Black, Piece::Pawn) => '♟',
    }
}
