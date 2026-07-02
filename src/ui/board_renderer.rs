use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::Paragraph,
};
use crate::engine::game::Game;
use crate::engine::board::{Piece, Color as PieceColor};
use crate::states::UiState;
use super::theme::Theme;

pub fn render_board(frame: &mut Frame, area: Rect, game: &Game, ui_state: &UiState, theme: &Theme) {
    let cell_width = area.width / 8;
    let cell_height = area.height / 8;
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

            let piece_str = if let Some((color, piece)) = piece_info {
                piece_to_unicode(color, piece)
            } else {
                ' '
            };

            let cell_area = Rect {
                x: area.x + file as u16 * cell_width,
                y: area.y + rank as u16 * cell_height,
                width: cell_width,
                height: cell_height,
            };

            let text = format!("{:^width$}", piece_str, width = cell_width as usize);
            let span = Span::styled(text, Style::default().fg(theme.text).bg(bg));
            frame.render_widget(Paragraph::new(span), cell_area);
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
