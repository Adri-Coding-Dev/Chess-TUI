use crate::engine::board::{Piece, Color, Square};
use crate::engine::moves::Move;

#[derive(Debug, Clone)]
pub enum UiState {
    Idle,
    PieceSelected {
        piece: (Color, Piece),
        from: Square,
        legal_moves: Vec<Move>,
    },
    MoveAnimation,
    WaitingOpponent,
    GameFinished,
    // Additional states for menus
    MainMenu,
    HistoryBrowser,
    Settings,
}
