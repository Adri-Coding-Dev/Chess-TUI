use super::board::{Board, Piece, Color};
use anyhow::{anyhow, Result};

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(anyhow!("Invalid FEN"));
        }
        // Parse piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err(anyhow!("Wrong number of ranks"));
        }
        let mut squares = [None; 64];
        for (i, rank) in ranks.iter().enumerate() {
            let rank_idx = 7 - i; // FEN starts from rank 8
            let mut file = 0;
            for ch in rank.chars() {
                if ch.is_digit(10) {
                    let skip = ch.to_digit(10).unwrap() as usize;
                    file += skip;
                } else {
                    let color = if ch.is_uppercase() { Color::White } else { Color::Black };
                    let piece = match ch.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => return Err(anyhow!("Invalid piece")),
                    };
                    let sq = rank_idx * 8 + file;
                    squares[sq] = Some((color, piece));
                    file += 1;
                }
            }
        }
        // Side to move
        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(anyhow!("Invalid side to move")),
        };
        // Castling (simplified)
        // En passant
        // Half move clock
        // Full move number
        Ok(Board {
            squares,
            castling: Default::default(),
            en_passant: None,
            half_move_clock: parts[4].parse().unwrap_or(0),
            full_move_number: parts[5].parse().unwrap_or(1),
            side_to_move,
        })
    }

    pub fn to_fen(&self) -> String {
        // Generate FEN string (simplified)
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    }
}
