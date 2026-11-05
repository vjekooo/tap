use crate::{browser::Browser, player::Player};
use ratatui::crossterm::event::KeyCode;
use rodio::OutputStreamHandle;
use std::path::{Path, PathBuf};

pub enum Mode {
    Player,
    Browser,
}

pub struct App {
    pub mode: Mode,
    pub browser: Browser,
    pub player: Player,
    pub message: String,
}

impl App {
    pub fn new(start_dir: PathBuf, handle: OutputStreamHandle) -> Self {
        Self {
            mode: Mode::Browser,
            browser: Browser::new(start_dir),
            player: Player::new(handle),
            message: String::new(),
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        self.play_queue(vec![path.to_path_buf()], 0);
    }

    fn play_queue(&mut self, queue: Vec<PathBuf>, index: usize) {
        match self.player.load_queue(queue, index) {
            Ok(()) => {
                self.message = format!("Loaded {}", self.player.name);
                self.mode = Mode::Player;
            }
            Err(e) => self.message = e,
        }
    }

    pub fn tick(&mut self) {
        if let Err(e) = self.player.tick() {
            self.message = e;
        }
    }

    /// Returns true when the app should quit.
    pub fn handle_key(&mut self, code: KeyCode) -> bool {
        if code == KeyCode::Char('q') {
            return true;
        }
        match self.mode {
            Mode::Player => match code {
                KeyCode::Char('o') => self.mode = Mode::Browser,
                KeyCode::Char(' ') => self.player.toggle_pause(),
                KeyCode::Char('s') => self.player.stop(),
                KeyCode::Char('n') => {
                    let _ = self.player.next();
                }
                KeyCode::Char('p') => {
                    let _ = self.player.prev();
                }
                KeyCode::Left => self.player.seek(false),
                KeyCode::Right => self.player.seek(true),
                KeyCode::Up => self.player.change_volume(0.1),
                KeyCode::Down => self.player.change_volume(-0.1),
                _ => {}
            },
            Mode::Browser => match code {
                KeyCode::Down | KeyCode::Char('j') => self.browser.next(),
                KeyCode::Up | KeyCode::Char('k') => self.browser.prev(),
                KeyCode::Left | KeyCode::Backspace | KeyCode::Char('h') => self.browser.go_up(),
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    if let Some(path) = self.browser.enter() {
                        self.load_file(&path);
                    }
                }
                KeyCode::Esc => {
                    if self.player.has_track() {
                        self.mode = Mode::Player;
                    }
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    if let Some(path) = self.browser.enter() {
                        let files = self.browser.audio_files();
                        let index = files.iter().position(|f| *f == path).unwrap_or(0);
                        self.play_queue(files, index);
                    }
                }
                KeyCode::Char('p') => {
                    let files = self.browser.audio_files();
                    if files.is_empty() {
                        self.message = "No audio files in this folder".into();
                    } else {
                        self.play_queue(files, 0);
                    }
                }
                _ => {}
            },
        }
        false
    }
}
