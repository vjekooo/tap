mod app;
mod browser;
mod player;
mod ui;

use app::App;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use rodio::OutputStream;
use std::{
    io,
    path::{Path, PathBuf},
    time::Duration,
};

fn main() -> io::Result<()> {
    let (_stream, handle) = OutputStream::try_default().map_err(|e| {
        eprintln!("Failed to initialize audio: {}", e);
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    let cwd = || std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let arg = std::env::args().nth(1).map(PathBuf::from);

    // Start the browser in the given directory, the file's directory, or the cwd
    let start_dir = match &arg {
        Some(p) if p.is_dir() => p.clone(),
        Some(p) => p
            .parent()
            .filter(|d| !d.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .unwrap_or_else(cwd),
        None => cwd(),
    };

    let mut app = App::new(start_dir, handle);

    // If a file was passed on the command line, play it right away
    if let Some(p) = arg.filter(|p| p.is_file()) {
        app.load_file(&p);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && app.handle_key(key.code) {
                    return Ok(());
                }
            }
        }
    }
}
