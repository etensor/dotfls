use crate::{
    app::{App, Mode},
    timer::fmt_dur,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use figlet_rs::FIGfont;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
        .split(size);

    // Queue panel
    let items: Vec<ListItem> = if app.queue.is_empty() {
        vec![ListItem::new("Queue empty")]
    } else {
        app.queue
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let mark = if i == 0 && app.running { "▶" } else { " " };
                let text = format!("{} {}. {} - {}", mark, i, t.label, fmt_dur(t.left));
                let mut item = ListItem::new(text);
                if i == app.selected {
                    item = item.style(
                        Style::default()
                            .fg(Color::Red)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    );
                }
                if i == 0 && app.running {
                    item = item.style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    );
                }
                item
            })
            .collect()
    };
    let left = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Queue"));
    f.render_widget(left, chunks[0]);

    // Right panel layout
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(5), // desc box
            Constraint::Min(10),   // remaining big
        ])
        .split(chunks[1]);

    // Title at top
    let title = Paragraph::new("RatTime")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).bg(Color::Black).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, right_chunks[0]);

    // Current + Description box
    if let Some(t) = app.queue.front() {
        let header = format!(" {}  |  Description", t.label);
        let desc_text = if t.description.is_empty() {
            "(none)".to_string()
        } else {
            t.description.clone()
        };
        let desc = Paragraph::new(desc_text)
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .block(Block::default().borders(Borders::ALL).title(header));
        f.render_widget(desc, right_chunks[1]);

        // Remaining time with figlet
        let font = FIGfont::standard().unwrap();
        let ascii = font.convert(&fmt_dur(t.left)).unwrap();
        let ascii_text = format!("\n{}", ascii); // padding above

        let remaining = Paragraph::new(ascii_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Remaining"));
        f.render_widget(remaining, right_chunks[2]);
    }

    // Input modal (Add / Edit / Description)
    if matches!(app.mode, Mode::Adding | Mode::Editing(_) | Mode::Description(_)) {
        let area = Rect {
            x: size.width / 4,
            y: size.height / 3,
            width: size.width / 2,
            height: 5,
        };

        let title = match app.mode {
            Mode::Adding => "Add Timer",
            Mode::Editing(_) => "Edit Timer",
            Mode::Description(_) => "Add Description",
            _ => "Input",
        };
        let modal = Paragraph::new(app.input.as_str())
            .style(Style::default().fg(Color::White).bg(Color::Black)) // solid bg for readability
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(modal, area);
    }

    // Alarm flash
    if let Some(start) = app.alarm_flash {
        if start.elapsed().as_secs() < 3 {
            let area = Rect {
                x: size.width / 4,
                y: size.height / 2 - 2,
                width: size.width / 2,
                height: 3,
            };
            let alarm = Paragraph::new("!!! ALARM !!!")
                .style(Style::default().fg(Color::Red).bg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Done"));
            f.render_widget(alarm, area);
        }
    }

    // Status bar
    let status_area = Rect {
        x: 0,
        y: size.height - 1,
        width: size.width,
        height: 1,
    };

    let style = match app.mode {
        Mode::Normal => Style::default().fg(Color::White).bg(Color::Blue),
        Mode::Adding => Style::default().fg(Color::Black).bg(Color::Green),
        Mode::Editing(_) => Style::default().fg(Color::Black).bg(Color::Yellow),
        Mode::Description(_) => Style::default().fg(Color::Black).bg(Color::Cyan),
    };

    let status_text = format!(
        "Mode: {:?} | ↑/↓=move, e=edit, r=remove, d=desc, a=add, SPACE=start/pause, q=quit",
        app.mode
    );

    let status = Paragraph::new(status_text).style(style);
    f.render_widget(status, status_area);
}
