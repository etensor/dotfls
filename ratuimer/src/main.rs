mod app;
mod timer;
mod ui;
mod audio;

use crate::app::{App, Event};
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, sync::mpsc, thread, time::{Duration, Instant}};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_default();
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if tx.send(Event::Tick).is_err() { break; }
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;

        match rx.recv()? {
            Event::Input(key) => app.handle_input(key),
            Event::Tick => app.tick(),
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
