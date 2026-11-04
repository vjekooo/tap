use ratatui::widgets::ListState;
use std::{
    fs,
    path::{Path, PathBuf},
};

const AUDIO_EXTS: [&str; 4] = ["mp3", "wav", "flac", "ogg"];

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub struct Browser {
    pub dir: PathBuf,
    pub entries: Vec<Entry>,
    pub state: ListState,
}

impl Browser {
    pub fn new(dir: PathBuf) -> Self {
        let dir = fs::canonicalize(&dir).unwrap_or(dir);
        let mut b = Browser {
            dir,
            entries: Vec::new(),
            state: ListState::default(),
        };
        b.refresh();
        b
    }

    fn refresh(&mut self) {
        let mut entries = Vec::new();
        if let Some(parent) = self.dir.parent() {
            entries.push(Entry {
                name: "..".into(),
                path: parent.to_path_buf(),
                is_dir: true,
            });
        }
        if let Ok(read_dir) = fs::read_dir(&self.dir) {
            let mut items: Vec<Entry> = read_dir
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let path = e.path();
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.starts_with('.') {
                        return None;
                    }
                    let is_dir = path.is_dir();
                    if is_dir || is_audio(&path) {
                        Some(Entry { name, path, is_dir })
                    } else {
                        None
                    }
                })
                .collect();
            // directories first, then alphabetical
            items.sort_by(|a, b| {
                b.is_dir
                    .cmp(&a.is_dir)
                    .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
            entries.extend(items);
        }
        self.entries = entries;
        self.state.select(if self.entries.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn next(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some((i + 1).min(self.entries.len() - 1)));
    }

    pub fn prev(&mut self) {
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some(i.saturating_sub(1)));
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.dir.parent() {
            self.dir = parent.to_path_buf();
            self.refresh();
        }
    }

    pub fn enter(&mut self) -> Option<PathBuf> {
        let entry = self.entries.get(self.state.selected()?)?;
        if entry.is_dir {
            self.dir = entry.path.clone();
            self.refresh();
            None
        } else {
            Some(entry.path.clone())
        }
    }
}
