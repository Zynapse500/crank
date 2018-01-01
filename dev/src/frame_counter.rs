

use std::time::Instant;

pub struct FrameCounter {
    start: Instant,
    frames: u32,
    fps: u32
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter {
            start: Instant::now(),
            frames: 0,
            fps: 0
        }
    }


    pub fn tick(&mut self) -> Option<u32> {
        let now = Instant::now();
        let elapsed = now - self.start;
        let elapsed_secs = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;

        self.frames += 1;

        if elapsed_secs > 0.5 {
            self.fps = (self.frames as f32 / elapsed_secs).round() as u32;
            self.start = now;
            self.frames = 0;

            return Some(self.fps);
        }

        None
    }


    pub fn get_rate(&self) -> u32 {
        self.fps
    }
}

