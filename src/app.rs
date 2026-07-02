use ratatui::{
    Frame,
};
use crate::config::Config;
use crate::engine::game::Game;
use crate::engine::board::{GameStatus, Color};
use crate::events::Event;
use crate::states::UiState;
use crate::ui::{layout, board_renderer, widgets, theme};
use crate::ui::layout::BoardLayout;
use crate::utils::time::ChessClock;
use crate::storage::history::HistoryEntry;
use std::time::Duration;

pub struct App {
    pub config: Config,
    pub game: Game,
    pub ui_state: UiState,
    pub clock: ChessClock,
    pub move_history: Vec<String>,
    pub current_turn: Color,
    pub game_result: Option<String>,
    quit: bool,
    pub history: Vec<HistoryEntry>,
    last_board: Option<BoardLayout>,
    last_left_rect: Option<ratatui::layout::Rect>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let game = Game::new();
        let clock = ChessClock::new(Duration::from_secs(config.time_minutes * 60));
        let history = crate::storage::history::load_history().unwrap_or_default();
        App {
            config,
            game,
            ui_state: UiState::Idle,
            clock,
            move_history: Vec::new(),
            current_turn: Color::White,
            game_result: None,
            quit: false,
            history,
            last_board: None,
            last_left_rect: None,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            Event::Key(key) => {
                if key.code == crossterm::event::KeyCode::Esc {
                    self.quit = true;
                }
            }
            Event::Tick => {
                self.clock.tick();
                if self.clock.white_time.is_zero() || self.clock.black_time.is_zero() {
                    self.game_result = Some("Time forfeit".into());
                }
                if !matches!(self.ui_state, UiState::WaitingOpponent) {
                    if self.config.ai_level.is_some() && self.game.current_turn() == Color::Black && self.game.status() == GameStatus::Ongoing {
                        self.make_ai_move();
                    }
                }
            }
            Event::Resize => {}
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        use crossterm::event::{MouseButton, MouseEventKind};
        if mouse.kind != MouseEventKind::Down(MouseButton::Left) {
            return;
        }
        let (col, row) = (mouse.column, mouse.row);
        let board = self.last_board;
        if let Some(board) = board {
            if board.rect.contains((col, row).into()) {
                self.handle_board_click(col, row, &board);
                return;
            }
        }
    }

    fn handle_board_click(&mut self, col: u16, row: u16, board: &BoardLayout) {
        let square = crate::ui::mouse::pixel_to_square(col, row, board);
        if square.is_none() {
            return;
        }
        let sq = square.unwrap();
        match &self.ui_state {
            UiState::Idle => {
                if let Some((color, _)) = self.game.board().piece_at(sq) {
                    if color == self.game.current_turn() {
                        let moves = self.game.legal_moves_from(sq);
                        if moves.is_empty() {
                            return;
                        }
                        self.ui_state = UiState::PieceSelected {
                            piece: (color, self.game.board().piece_at(sq).unwrap().1),
                            from: sq,
                            legal_moves: moves,
                        };
                    }
                }
            }
            UiState::PieceSelected { from, legal_moves, .. } => {
                if sq == *from {
                    self.ui_state = UiState::Idle;
                } else if let Some(mov) = legal_moves.iter().find(|m| m.to == sq) {
                    let mov = mov.clone();
                    self.make_move(mov);
                } else if let Some((color, _)) = self.game.board().piece_at(sq) {
                    if color == self.game.current_turn() {
                        let moves = self.game.legal_moves_from(sq);
                        if moves.is_empty() {
                            self.ui_state = UiState::Idle;
                            return;
                        }
                        self.ui_state = UiState::PieceSelected {
                            piece: (color, self.game.board().piece_at(sq).unwrap().1),
                            from: sq,
                            legal_moves: moves,
                        };
                    }
                }
            }
            _ => {}
        }
    }

    fn make_move(&mut self, mov: crate::engine::moves::Move) {
        if self.game.make_move(&mov).is_err() {
            return;
        }
        self.move_history.push(mov.to_algebraic(&self.game.board()));
        self.clock.switch_turn();
        self.ui_state = UiState::Idle;

        match self.game.status() {
            GameStatus::Checkmate => self.game_result = Some(format!("{:?} wins by checkmate", self.game.current_turn().opposite())),
            GameStatus::Stalemate => self.game_result = Some("Stalemate - Draw".into()),
            GameStatus::Draw => self.game_result = Some("Draw".into()),
            _ => {}
        }
    }

    fn make_ai_move(&mut self) {
        let ai = crate::ai::get_ai(self.config.ai_level.unwrap_or(1));
        if let Some(best_move) = ai.next_move(&self.game) {
            self.make_move(best_move);
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let theme = theme::tokio_night_theme();
        let chunks = layout::create_layout(f.size(), &theme);
        self.last_board = Some(chunks.board);
        self.last_left_rect = Some(chunks.left);

        widgets::render_left_panel(f, chunks.left, &self.history, &self.config, &theme);
        board_renderer::render_board(f, &chunks.board, &self.game, &self.ui_state, &theme);
        widgets::render_right_panel(f, chunks.right, &self.game, &self.clock, &self.move_history, &self.config, &theme);
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }
}

pub type AppResult<T> = std::result::Result<T, anyhow::Error>;
