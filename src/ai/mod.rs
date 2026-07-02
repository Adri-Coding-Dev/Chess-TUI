use crate::engine::game::Game;
use crate::engine::moves::Move;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait ChessAi {
    fn next_move(&self, game: &Game) -> Option<Move>;
}

pub struct RandomAi;

impl ChessAi for RandomAi {
    fn next_move(&self, game: &Game) -> Option<Move> {
        let mut rng = thread_rng();
        game.legal_moves().choose(&mut rng).cloned()
    }
}

// Additional AI levels can be added here

pub fn get_ai(level: u8) -> Box<dyn ChessAi> {
    match level {
        1 => Box::new(RandomAi),
        _ => Box::new(RandomAi),
    }
}
