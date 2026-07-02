use serde::{Serialize, Deserialize};

pub type Square = usize; // 0..63, a1=0, h8=63

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub squares: [Option<(Color, Piece)>; 64],
    pub castling: CastlingRights,
    pub en_passant: Option<Square>,
    pub half_move_clock: u32,
    pub full_move_number: u32,
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn piece_at(&self, sq: Square) -> Option<(Color, Piece)> {
        self.squares[sq]
    }

    pub fn set_piece(&mut self, sq: Square, piece: Option<(Color, Piece)>) {
        self.squares[sq] = piece;
    }

    pub fn is_empty(&self, sq: Square) -> bool {
        self.squares[sq].is_none()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn switch_turn(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }

    pub fn apply_move(&mut self, m: &crate::engine::moves::Move) -> Result<(), String> {
        let piece = self.squares[m.from].ok_or("No piece")?;
        // Handle castling
        if let Some(castling) = m.is_castling {
            match (piece.0, castling) {
                (Color::White, crate::engine::moves::CastlingType::KingSide) => {
                    self.squares[5] = self.squares[4].take(); // king
                    self.squares[4] = None;
                    self.squares[6] = self.squares[7].take(); // rook
                    self.squares[7] = None;
                    self.castling.white_king = false;
                    self.castling.white_queen = false;
                    return Ok(());
                }
                // ... similar for queen side and black
                _ => return Err("Invalid castling".into()),
            }
        }
        // En passant capture
        if m.is_en_passant {
            let captured_sq = if piece.0 == Color::White { m.to - 8 } else { m.to + 8 };
            self.squares[captured_sq] = None;
        }
        // Normal move
        self.squares[m.to] = Some(piece);
        self.squares[m.from] = None;
        // Promotion
        if let Some(prom) = m.promotion {
            self.squares[m.to] = Some((piece.0, prom));
        }
        // Update castling rights
        if piece.1 == Piece::King {
            match piece.0 {
                Color::White => { self.castling.white_king = false; self.castling.white_queen = false; }
                Color::Black => { self.castling.black_king = false; self.castling.black_queen = false; }
            }
        } else if piece.1 == Piece::Rook {
            match (piece.0, m.from) {
                (Color::White, 0) => self.castling.white_queen = false,
                (Color::White, 7) => self.castling.white_king = false,
                (Color::Black, 56) => self.castling.black_queen = false,
                (Color::Black, 63) => self.castling.black_king = false,
                _ => {}
            }
        }
        // En passant target
        self.en_passant = None;
        if piece.1 == Piece::Pawn && (m.to as i32 - m.from as i32).abs() == 16 {
            self.en_passant = Some((m.from + m.to) / 2);
        }
        self.half_move_clock = if piece.1 == Piece::Pawn || self.squares[m.to].is_some() { 0 } else { self.half_move_clock + 1 };
        if piece.0 == Color::Black {
            self.full_move_number += 1;
        }
        Ok(())
    }
}

pub fn square_to_file_rank(sq: Square) -> (u8, u8) {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;
    (file, rank)
}

pub fn file_rank_to_square(file: u8, rank: u8) -> Square {
    rank as usize * 8 + file as usize
}
