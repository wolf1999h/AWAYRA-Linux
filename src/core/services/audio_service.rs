use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rodio::{OutputStream, Sink, source::SineWave, Source};
use crate::core::models::{AppSettings, BreakSoundTheme, BreakType};

/// Audio service supporting synthesised themes, volume control, repeating chime playback, and sound previews.
pub struct AudioService {
    active_repeat_stop: Arc<Mutex<Option<std::sync::mpsc::Sender<()>>>>,
}

impl AudioService {
    pub fn new() -> Self {
        Self {
            active_repeat_stop: Arc::new(Mutex::new(None)),
        }
    }

    /// Stop any active repeating playback loop
    pub fn stop_repeating(&self) {
        if let Ok(mut lock) = self.active_repeat_stop.lock() {
            if let Some(tx) = lock.take() {
                let _ = tx.send(());
            }
        }
    }

    /// Play sound when a break starts based on user settings
    pub fn play_break_start(&self, break_type: BreakType, settings: &AppSettings) {
        let enabled = match break_type {
            BreakType::Eye => settings.eye_break_sound_enabled,
            BreakType::Move => settings.move_break_sound_enabled,
        };
        if !enabled || settings.break_sound_volume <= 0 {
            return;
        }

        self.stop_repeating();

        let theme = settings.break_sound_theme;
        let volume = (settings.break_sound_volume.clamp(0, 100) as f32) / 100.0;
        let repeat_secs = settings.break_sound_repeat_seconds;

        if repeat_secs > 0 {
            let (tx, rx) = std::sync::mpsc::channel::<()>();
            if let Ok(mut lock) = self.active_repeat_stop.lock() {
                *lock = Some(tx);
            }
            thread::spawn(move || {
                loop {
                    play_theme_sound(theme, volume);
                    // Wait for repeat interval or stop signal
                    if rx.recv_timeout(Duration::from_secs(repeat_secs as u64)).is_ok() {
                        break;
                    }
                }
            });
        } else {
            thread::spawn(move || {
                play_theme_sound(theme, volume);
            });
        }
    }

    /// Play sound when break ends (single notification tone)
    pub fn play_break_end(&self, settings: &AppSettings) {
        self.stop_repeating();
        if settings.break_sound_volume <= 0 {
            return;
        }
        let volume = (settings.break_sound_volume.clamp(0, 100) as f32) / 100.0;
        thread::spawn(move || {
            play_tone_sequence(&[(880.0, 0.15), (1174.66, 0.25)], volume);
        });
    }

    /// Preview a sound theme at a given volume
    pub fn preview_sound(&self, theme: BreakSoundTheme, volume: i32) {
        self.stop_repeating();
        let vol = (volume.clamp(0, 100) as f32) / 100.0;
        thread::spawn(move || {
            play_theme_sound(theme, vol);
        });
    }
}

/// Helper function to play melodic themes synthesized using sine/harmonic waves
fn play_theme_sound(theme: BreakSoundTheme, volume: f32) {
    let notes: Vec<(f32, f32)> = match theme {
        // Soft Bell: gentle dual-tone chime (A4 -> E5)
        BreakSoundTheme::SoftBell => vec![(440.0, 0.4), (659.25, 0.6)],

        // Gentle Chime: 3-note ascending arpeggio (C5 -> E5 -> G5)
        BreakSoundTheme::GentleChime => vec![(523.25, 0.25), (659.25, 0.25), (783.99, 0.4)],

        // Calm Drop: soft descending pair (G4 -> D4)
        BreakSoundTheme::CalmDrop => vec![(392.0, 0.35), (293.66, 0.5)],

        // Calm Piano: pentatonic triad (F4 -> A4 -> C5 -> F5)
        BreakSoundTheme::CalmPiano => vec![(349.23, 0.2), (440.0, 0.2), (523.25, 0.25), (698.46, 0.4)],

        // Morning Dew: bright high chime (E5 -> B5)
        BreakSoundTheme::MorningDew => vec![(659.25, 0.2), (987.77, 0.4)],

        // Still Water: serene low tone (D4 -> A4)
        BreakSoundTheme::StillWater => vec![(293.66, 0.5), (440.0, 0.6)],
    };

    play_tone_sequence(&notes, volume);
}

fn play_tone_sequence(notes: &[(f32, f32)], volume: f32) {
    let Ok((stream, stream_handle)) = OutputStream::try_default() else { return; };
    let Ok(sink) = Sink::try_new(&stream_handle) else { return; };

    sink.set_volume(volume);
    for &(freq, dur_sec) in notes {
        let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(dur_sec));
        sink.append(source);
    }
    sink.sleep_until_end();
    thread::sleep(Duration::from_millis(50));
    drop(sink);
    drop(stream);
}
