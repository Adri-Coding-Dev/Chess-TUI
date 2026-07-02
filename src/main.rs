mod app;
mod config;
mod events;
mod states;
mod engine;
mod ai;
mod ui;
mod network;
mod storage;
mod utils;

use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, Event as CrosstermEvent, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use app::{App, AppResult};
use events::Event;
use config::Config;

fn main() -> AppResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config::load().unwrap_or_default();
    let mut app = App::new(config);

    let tick_rate = Duration::from_millis(100);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    app.config.save().ok();

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> AppResult<()> {
    let mut last_tick = std::time::Instant::now();
    loop {
        terminal.draw(|f| app.render(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            match event::read()? {
                CrosstermEvent::Mouse(mouse) => app.handle_event(Event::Mouse(mouse)),
                CrosstermEvent::Key(key) => app.handle_event(Event::Key(key)),
                CrosstermEvent::Resize(_, _) => app.handle_event(Event::Resize),
                _ => {}
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.handle_event(Event::Tick);
            last_tick = std::time::Instant::now();
        }

        if app.should_quit() {
            return Ok(());
        }
    }
}
