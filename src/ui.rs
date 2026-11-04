use crate::app::{App, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};
use std::time::Duration;

fn fmt_time(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}", s / 60, s % 60)
}

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.mode {
        Mode::Player => draw_player(f, app),
        Mode::Browser => draw_browser(f, app),
    }
}

fn draw_player(f: &mut Frame, app: &App) {
    let p = &app.player;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let name = if p.name.is_empty() {
        "-".to_string()
    } else {
        p.info.display(&p.name)
    };
    let title = Paragraph::new(format!(
        "{}  |  {}  |  Vol {}%",
        name,
        p.status(),
        (p.volume * 100.0).round() as u32
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Terminal Audio Player "),
    );
    f.render_widget(title, chunks[0]);

    let pos = p.position();
    let (ratio, label) = if p.total.is_zero() {
        (0.0, fmt_time(pos))
    } else {
        (
            (pos.as_secs_f64() / p.total.as_secs_f64()).clamp(0.0, 1.0),
            format!("{} / {}", fmt_time(pos), fmt_time(p.total)),
        )
    };
    let progress = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::LightCyan))
        .label(label)
        .ratio(ratio);
    f.render_widget(progress, chunks[1]);

    let controls =
        Paragraph::new("O=Open File  SPACE=Play/Pause  S=Stop  ←/→=Seek  ↑/↓=Volume  Q=Quit")
            .block(Block::default().borders(Borders::ALL).title(" Controls "));
    f.render_widget(controls, chunks[2]);

    if let Some(album) = &p.info.album {
        f.render_widget(Paragraph::new(format!("Album: {album}")), chunks[3]);
    }
}

fn draw_browser(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    let items: Vec<ListItem> = app
        .browser
        .entries
        .iter()
        .map(|e| {
            if e.is_dir {
                ListItem::new(format!("{}/", e.name)).style(Style::default().fg(Color::Cyan))
            } else {
                ListItem::new(e.name.clone())
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Open: {} ", app.browser.dir.display())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunks[0], &mut app.browser.state);

    let help_text = if app.message.is_empty() {
        "Up/Down=Move  Enter=Open  Backspace=Parent dir  Esc=Back to player  Q=Quit".to_string()
    } else {
        app.message.clone()
    };
    let help =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(help, chunks[1]);
}
