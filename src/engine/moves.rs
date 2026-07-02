use super::board::{Square, Piece};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub is_en_passant: bool,
    pub is_castling: Option<CastlingType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingType {
    KingSide,
    QueenSide,
}

impl Move {
    pub fn simple(from: Square, to: Square) -> Self {
        Move {
            from,
            to,
            promotion: None,
            is_en_passant: false,
            is_castling: None,
        }
    }

    pub fn to_algebraic(&self, _board: &crate::engine::board::Board) -> String {
        let from_file = (self.from % 8) as u8 + b'a';
        let from_rank = (self.from / 8) as u8 + b'1';
        let to_file = (self.to % 8) as u8 + b'a';
        let to_rank = (self.to / 8) as u8 + b'1';
        let prom = match self.promotion {
            Some(Piece::Queen) => "q",
            Some(Piece::Rook) => "r",
            Some(Piece::Bishop) => "b",
            Some(Piece::Knight) => "n",
            _ => "",
        };
        format!("{}{}{}{}{}", from_file as char, from_rank as char, to_file as char, to_rank as char, prom)
    }
}
