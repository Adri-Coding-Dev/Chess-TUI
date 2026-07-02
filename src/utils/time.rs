use crate::engine::board::Color;
use std::time::{Duration, Instant};

pub struct ChessClock {
    pub white_time: Duration,
    pub black_time: Duration,
    active: Option<Color>,
    last_tick: Instant,
}

impl ChessClock {
    pub fn new(initial: Duration) -> Self {
        ChessClock {
            white_time: initial,
            black_time: initial,
            active: Some(Color::White),
            last_tick: Instant::now(),
        }
    }

    pub fn switch_turn(&mut self) {
        self.active = self.active.map(|c| c.opposite());
        self.last_tick = Instant::now();
    }

    pub fn tick(&mut self) {
        if let Some(color) = self.active {
            let elapsed = self.last_tick.elapsed();
            self.last_tick = Instant::now();
            match color {
                Color::White => {
                    self.white_time = self.white_time.saturating_sub(elapsed);
                }
                Color::Black => {
                    self.black_time = self.black_time.saturating_sub(elapsed);
                }
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.white_time.is_zero() || self.black_time.is_zero()
    }
}

// Override is_zero method for use in app.rs (already correct)
