use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::config::Config;
use crate::engine::game::Game;
use crate::utils::time::ChessClock;
use super::theme::Theme;
use crate::storage::history::HistoryEntry;

pub fn render_left_panel(frame: &mut Frame, area: Rect, history: &[HistoryEntry], _config: &Config, theme: &Theme) {
    let mut items: Vec<ListItem> = history.iter().map(|h| {
        let text = format!("{}  {} vs {}  {}", h.date.format("%Y-%m-%d"), h.white, h.black, h.result);
        ListItem::new(Span::styled(text, Style::default().fg(theme.text)))
    }).collect();
    if items.is_empty() {
        items.push(ListItem::new("No history"));
    }
    let list = List::new(items)
        .block(Block::default().title("Game History").borders(Borders::ALL));
    frame.render_widget(list, area);
}

pub fn render_right_panel(frame: &mut Frame, area: Rect, _game: &Game, clock: &ChessClock, move_history: &[String], config: &Config, theme: &Theme) {
    let white_time = format_time(clock.white_time);
    let black_time = format_time(clock.black_time);

    let text = vec![
        Line::from(Span::styled(format!("White: {} (ELO {})", config.player_name, 1200), Style::default().fg(theme.text))),
        Line::from(Span::styled(format!("Black: {} (ELO {})", "AI", 1000), Style::default().fg(theme.text))),
        Line::from(""),
        Line::from(Span::styled(format!("White clock: {}", white_time), Style::default().fg(theme.text))),
        Line::from(Span::styled(format!("Black clock: {}", black_time), Style::default().fg(theme.text))),
        Line::from(""),
        Line::from(Span::styled("Move history:", Style::default().fg(theme.text))),
    ];

    let mut history_lines: Vec<Line> = move_history.chunks(2).enumerate().map(|(i, pair)| {
        let white_move = pair.first().cloned().unwrap_or_default();
        let black_move = pair.get(1).cloned().unwrap_or_default();
        Line::from(format!("{}. {} {}", i+1, white_move, black_move))
    }).collect();
    let max_lines = area.height.saturating_sub(10) as usize;
    history_lines.truncate(max_lines);

    let paragraph = Paragraph::new(text.into_iter().chain(history_lines).collect::<Vec<_>>())
        .block(Block::default().title("Game Info").borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn format_time(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
