use std::{fs, path::PathBuf};

fn state_file() -> Option<PathBuf> {
    let mut p = dirs::config_dir()?;
    p.push("term-player");
    p.push("last_dir");
    Some(p)
}

/// Returns the saved directory, if it still exists.
pub fn load_last_dir() -> Option<PathBuf> {
    let text = fs::read_to_string(state_file()?).ok()?;
    let path = PathBuf::from(text.trim());
    path.is_dir().then_some(path)
}

pub fn save_last_dir(dir: &PathBuf) {
    let Some(file) = state_file() else { return };
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(file, dir.to_string_lossy().as_bytes());
}
