use std::thread;
use rodio::{OutputStream, Sink};

pub struct AudioService;

impl AudioService {
    pub fn new() -> Self {
        Self
    }

    pub fn play_break_start(&self) {
        self.play_beep(440.0, 0.3);
    }

    pub fn play_break_end(&self) {
        self.play_beep(880.0, 0.3);
    }

    fn play_beep(&self, frequency: f32, _duration_secs: f32) {
        let _ = thread::spawn(move || {
            let _ = (|| -> Result<(), ()> {
                if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                        let source = rodio::source::SineWave::new(frequency);
                        sink.append(source);
                        sink.sleep_until_end();
                    }
                }
                Ok(())
            })();
        });
    }
}
