use super::board::{Board, Square, Piece, Color, GameStatus};
use super::moves::{Move, CastlingType};

pub struct Game {
    board: Board,
    // position_history: Vec<u64>, // for threefold repetition (not yet implemented)
    // half_move_clock: u32,
    result: Option<GameStatus>,
    status: GameStatus,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        Game {
            board,
            result: None,
            status: GameStatus::Ongoing,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_turn(&self) -> Color {
        self.board.side_to_move
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = self.generate_pseudo_moves(self.current_turn());
        moves.retain(|m| {
            let mut copy = self.board.clone();
            if copy.apply_move(m).is_err() {
                return false;
            }
            !self.is_in_check_after_move(&copy, self.current_turn())
        });
        moves
    }

    pub fn legal_moves_from(&self, sq: Square) -> Vec<Move> {
        self.legal_moves().into_iter().filter(|m| m.from == sq).collect()
    }

    pub fn make_move(&mut self, m: &Move) -> Result<(), String> {
        if !self.legal_moves().contains(m) {
            return Err("Illegal move".to_string());
        }
        self.board.apply_move(m)?;
        self.board.switch_turn();
        self.status = self.compute_status();
        if self.status != GameStatus::Ongoing {
            self.result = Some(self.status);
        }
        Ok(())
    }

    fn compute_status(&self) -> GameStatus {
        let color = self.current_turn();
        let moves = self.legal_moves();
        if self.is_in_check(color) {
            if moves.is_empty() {
                GameStatus::Checkmate
            } else {
                GameStatus::Check
            }
        } else if moves.is_empty() {
            GameStatus::Stalemate
        } else {
            GameStatus::Ongoing
        }
    }

    fn is_in_check(&self, color: Color) -> bool {
        let king_sq = self.find_king(color);
        self.is_square_attacked(king_sq, color.opposite())
    }

    fn is_in_check_after_move(&self, board: &Board, color: Color) -> bool {
        let king_sq = board.squares.iter().position(|sq| {
            *sq == Some((color, Piece::King))
        }).unwrap();
        Self::is_square_attacked_by_color(board, king_sq, color.opposite())
    }

    fn find_king(&self, color: Color) -> Square {
        self.board.squares.iter().position(|sq| {
            *sq == Some((color, Piece::King))
        }).expect("King missing")
    }

    fn is_square_attacked(&self, sq: Square, attacker_color: Color) -> bool {
        Self::is_square_attacked_by_color(&self.board, sq, attacker_color)
    }

    fn is_square_attacked_by_color(board: &Board, sq: Square, attacker_color: Color) -> bool {
        let sq_i32 = sq as i32;
        // Pawn attacks
        let pawn_dir: i32 = if attacker_color == Color::White { -8 } else { 8 };
        for &offset in &[pawn_dir + 1, pawn_dir - 1] {
            let target = sq_i32 + offset;
            if target >= 0 && target < 64 {
                if board.squares[target as usize] == Some((attacker_color, Piece::Pawn)) {
                    return true;
                }
            }
        }
        // Knight attacks
        let knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17];
        for &offset in &knight_offsets {
            let target = sq_i32 + offset;
            if target >= 0 && target < 64 {
                let target = target as usize;
                let dx = (sq_i32 % 8 - target as i32 % 8).abs();
                let dy = (sq_i32 / 8 - target as i32 / 8).abs();
                if (dx == 1 && dy == 2) || (dx == 2 && dy == 1) {
                    if board.squares[target] == Some((attacker_color, Piece::Knight)) {
                        return true;
                    }
                }
            }
        }
        // Sliding attacks (bishop/rook/queen) and king (one step)
        let directions = [
            (1,0), (-1,0), (0,1), (0,-1),
            (1,1), (1,-1), (-1,1), (-1,-1),
        ];
        for &(df, dr) in &directions {
            let mut f = (sq % 8) as i32;
            let mut r = (sq / 8) as i32;
            loop {
                f += df;
                r += dr;
                if f < 0 || f > 7 || r < 0 || r > 7 { break; }
                let idx = (r * 8 + f) as usize;
                if let Some((c, p)) = board.squares[idx] {
                    if c == attacker_color {
                        let is_rook = df == 0 || dr == 0;
                        let is_bishop = df != 0 && dr != 0;
                        if p == Piece::Queen ||
                           (p == Piece::Rook && is_rook) ||
                           (p == Piece::Bishop && is_bishop) ||
                           (p == Piece::King && (df.abs() <= 1 && dr.abs() <= 1)) {
                            return true;
                        }
                    }
                    break;
                }
            }
        }
        false
    }

    fn generate_pseudo_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for sq in 0..64 {
            if let Some((pc, piece)) = self.board.piece_at(sq) {
                if pc == color {
                    self.generate_piece_moves(sq, pc, piece, &mut moves);
                }
            }
        }
        // Castling
        if color == Color::White {
            if self.board.castling.white_king {
                if self.board.squares[5].is_none() && self.board.squares[6].is_none() &&
                   self.board.squares[4] == Some((Color::White, Piece::King)) &&
                   self.board.squares[7] == Some((Color::White, Piece::Rook)) &&
                   !self.is_square_attacked(4, Color::Black) &&
                   !self.is_square_attacked(5, Color::Black) &&
                   !self.is_square_attacked(6, Color::Black) {
                    moves.push(Move {
                        from: 4,
                        to: 6,
                        promotion: None,
                        is_en_passant: false,
                        is_castling: Some(CastlingType::KingSide),
                    });
                }
            }
            if self.board.castling.white_queen {
                if self.board.squares[3].is_none() && self.board.squares[2].is_none() && self.board.squares[1].is_none() &&
                   self.board.squares[4] == Some((Color::White, Piece::King)) &&
                   self.board.squares[0] == Some((Color::White, Piece::Rook)) &&
                   !self.is_square_attacked(4, Color::Black) &&
                   !self.is_square_attacked(3, Color::Black) &&
                   !self.is_square_attacked(2, Color::Black) {
                    moves.push(Move {
                        from: 4,
                        to: 2,
                        promotion: None,
                        is_en_passant: false,
                        is_castling: Some(CastlingType::QueenSide),
                    });
                }
            }
        } else {
            if self.board.castling.black_king {
                if self.board.squares[61].is_none() && self.board.squares[62].is_none() &&
                   self.board.squares[60] == Some((Color::Black, Piece::King)) &&
                   self.board.squares[63] == Some((Color::Black, Piece::Rook)) &&
                   !self.is_square_attacked(60, Color::White) &&
                   !self.is_square_attacked(61, Color::White) &&
                   !self.is_square_attacked(62, Color::White) {
                    moves.push(Move {
                        from: 60,
                        to: 62,
                        promotion: None,
                        is_en_passant: false,
                        is_castling: Some(CastlingType::KingSide),
                    });
                }
            }
            if self.board.castling.black_queen {
                if self.board.squares[59].is_none() && self.board.squares[58].is_none() && self.board.squares[57].is_none() &&
                   self.board.squares[60] == Some((Color::Black, Piece::King)) &&
                   self.board.squares[56] == Some((Color::Black, Piece::Rook)) &&
                   !self.is_square_attacked(60, Color::White) &&
                   !self.is_square_attacked(59, Color::White) &&
                   !self.is_square_attacked(58, Color::White) {
                    moves.push(Move {
                        from: 60,
                        to: 58,
                        promotion: None,
                        is_en_passant: false,
                        is_castling: Some(CastlingType::QueenSide),
                    });
                }
            }
        }
        moves
    }

    fn generate_piece_moves(&self, from: Square, color: Color, piece: Piece, moves: &mut Vec<Move>) {
        match piece {
            Piece::Pawn => self.pawn_moves(from, color, moves),
            Piece::Knight => self.knight_moves(from, color, moves),
            Piece::Bishop => self.sliding_moves(from, color, &[(1,1), (1,-1), (-1,1), (-1,-1)], moves),
            Piece::Rook => self.sliding_moves(from, color, &[(0,1), (0,-1), (1,0), (-1,0)], moves),
            Piece::Queen => self.sliding_moves(from, color, &[(1,1),(1,-1),(-1,1),(-1,-1),(0,1),(0,-1),(1,0),(-1,0)], moves),
            Piece::King => self.king_moves(from, color, moves),
        }
    }

    fn pawn_moves(&self, from: Square, color: Color, moves: &mut Vec<Move>) {
        let direction: i32 = if color == Color::White { 8 } else { -8 };
        let start_rank = if color == Color::White { 1 } else { 6 };
        let to = (from as i32 + direction) as usize;
        if self.board.is_empty(to) {
            let promotion_rank = if color == Color::White { 7 } else { 0 };
            if (to / 8) == promotion_rank {
                for &prom in &[Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(prom), is_en_passant: false, is_castling: None });
                }
            } else {
                moves.push(Move::simple(from, to));
            }
            // double push
            if from / 8 == start_rank {
                let to2 = (from as i32 + 2*direction) as usize;
                if self.board.is_empty(to2) {
                    moves.push(Move::simple(from, to2));
                }
            }
        }
        // captures
        for &offset in &[direction + 1, direction - 1] {
            let target = from as i32 + offset;
            if target >= 0 && target < 64 {
                let to = target as usize;
                if let Some((c, _)) = self.board.piece_at(to) {
                    if c != color {
                        if (to / 8) == (if color == Color::White { 7 } else { 0 }) {
                            for &prom in &[Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                                moves.push(Move { from, to, promotion: Some(prom), is_en_passant: false, is_castling: None });
                            }
                        } else {
                            moves.push(Move::simple(from, to));
                        }
                    }
                }
                // En passant
                if let Some(ep) = self.board.en_passant {
                    if target == ep as i32 {
                        moves.push(Move { from, to: ep, promotion: None, is_en_passant: true, is_castling: None });
                    }
                }
            }
        }
    }

    fn knight_moves(&self, from: Square, _color: Color, moves: &mut Vec<Move>) {
        let offsets = [-17, -15, -10, -6, 6, 10, 15, 17];
        for &off in &offsets {
            let target = from as i32 + off;
            if target >= 0 && target < 64 {
                let to = target as usize;
                let dx = (from as i32 % 8 - to as i32 % 8).abs();
                let dy = (from as i32 / 8 - to as i32 / 8).abs();
                if (dx == 1 && dy == 2) || (dx == 2 && dy == 1) {
                    if self.board.is_empty(to) || self.board.piece_at(to).unwrap().0 != _color {
                        moves.push(Move::simple(from, to));
                    }
                }
            }
        }
    }

    fn sliding_moves(&self, from: Square, color: Color, directions: &[(i32, i32)], moves: &mut Vec<Move>) {
        for &(df, dr) in directions {
            let mut f = (from % 8) as i32;
            let mut r = (from / 8) as i32;
            loop {
                f += df;
                r += dr;
                if f < 0 || f > 7 || r < 0 || r > 7 { break; }
                let to = (r * 8 + f) as usize;
                if let Some((c, _)) = self.board.piece_at(to) {
                    if c != color {
                        moves.push(Move::simple(from, to));
                    }
                    break;
                }
                moves.push(Move::simple(from, to));
            }
        }
    }

    fn king_moves(&self, from: Square, color: Color, moves: &mut Vec<Move>) {
        let offsets = [-9, -8, -7, -1, 1, 7, 8, 9];
        for &off in &offsets {
            let target = from as i32 + off;
            if target >= 0 && target < 64 {
                let to = target as usize;
                let dx = (from as i32 % 8 - to as i32 % 8).abs();
                let dy = (from as i32 / 8 - to as i32 / 8).abs();
                if dx <= 1 && dy <= 1 {
                    if self.board.is_empty(to) || self.board.piece_at(to).unwrap().0 != color {
                        moves.push(Move::simple(from, to));
                    }
                }
            }
        }
    }
}
