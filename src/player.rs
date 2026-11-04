use crate::metadata::{self, TrackInfo};
use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use std::{fs::File, io::BufReader, path::Path, time::Duration};

pub struct Player {
    handle: OutputStreamHandle,
    sink: Option<Sink>,
    pub name: String,
    pub total: Duration,
    pub volume: f32,
    pub info: TrackInfo,
}

impl Player {
    pub fn new(handle: OutputStreamHandle) -> Self {
        Self {
            handle,
            sink: None,
            name: String::new(),
            total: Duration::ZERO,
            volume: 1.0,
            info: TrackInfo::default(),
        }
    }

    pub fn has_track(&self) -> bool {
        self.sink.is_some()
    }

    pub fn load(&mut self, path: &Path) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {e}"))?;
        let decoder = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode file: {e}"))?;
        let source = decoder.convert_samples::<f32>();
        let total = source.total_duration().unwrap_or_default();

        // A fresh sink per track is the simplest way to fully reset state
        let sink = Sink::try_new(&self.handle).map_err(|e| format!("Audio error: {e}"))?;
        sink.set_volume(self.volume);
        sink.append(source);
        sink.play();

        if let Some(old) = self.sink.take() {
            old.stop();
        }
        self.sink = Some(sink);
        self.total = total;
        self.name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        self.info = metadata::read(path);
        Ok(())
    }

    pub fn toggle_pause(&self) {
        if let Some(sink) = &self.sink {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    pub fn seek(&self, forward: bool) {
        let Some(sink) = &self.sink else { return };
        let step = Duration::from_secs(10);
        let pos = sink.get_pos();
        let target = if forward {
            let t = pos + step;
            if self.total.is_zero() {
                t
            } else {
                t.min(self.total)
            }
        } else {
            pos.saturating_sub(step)
        };
        let _ = sink.try_seek(target); // not every source is seekable
    }

    pub fn change_volume(&mut self, delta: f32) {
        self.volume = (self.volume + delta).clamp(0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn status(&self) -> &'static str {
        match &self.sink {
            None => "No track",
            Some(s) if s.empty() => "Stopped",
            Some(s) if s.is_paused() => "Paused",
            Some(_) => "Playing",
        }
    }

    pub fn position(&self) -> Duration {
        self.sink.as_ref().map(|s| s.get_pos()).unwrap_or_default()
    }
}
