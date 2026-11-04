use lofty::{file::TaggedFileExt, probe::Probe, tag::Accessor};
use std::path::Path;

#[derive(Default, Clone)]
pub struct TrackInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl TrackInfo {
    pub fn display(&self, fallback: &str) -> String {
        match (&self.artist, &self.title) {
            (Some(a), Some(t)) => format!("{a} - {t}"),
            (None, Some(t)) => t.clone(),
            _ => fallback.to_string(),
        }
    }
}

pub fn read(path: &Path) -> TrackInfo {
    let Ok(probe) = Probe::open(path) else {
        return TrackInfo::default();
    };
    let Ok(tagged) = probe.read() else {
        return TrackInfo::default();
    };

    let Some(tag) = tagged.primary_tag().or_else(|| tagged.first_tag()) else {
        return TrackInfo::default();
    };

    TrackInfo {
        title: tag.title().map(|s| s.to_string()),
        artist: tag.artist().map(|s| s.to_string()),
        album: tag.album().map(|s| s.to_string()),
    }
}
