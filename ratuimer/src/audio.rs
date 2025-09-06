use std::time::Duration;
use rodio::{OutputStreamBuilder, Sink, Source};

pub fn play_alarm() {
    if let Ok(builder) = OutputStreamBuilder::from_default_device() {
        if let Ok(stream) = builder.open_stream() {
            // Connect to mixer for playback
            let sink = Sink::connect_new(&stream.mixer());
            
            // 1-second sine wave beep (880 Hz)
            let source = rodio::source::SineWave::new(440.0)
                .take_duration(Duration::from_millis(750));
            sink.append(source);
            sink.detach(); // let it play asynchronously
            
            // Keep stream alive during playback
            std::mem::forget(stream);
            return;
        }
    }
    // Fallback: terminal bell if audio fails
    print!("\x07");
}
