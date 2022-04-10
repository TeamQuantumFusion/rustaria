use rustaria_util::info;
use std::time::{Duration, Instant};

pub struct Profiler {
    draw_calls: usize,

    frame_time: Duration,
    frames: usize,

    last_print: Instant,
    start_frame: Instant,
}

impl Profiler {
    pub fn new() -> Profiler {
        Profiler {
            draw_calls: 0,
            frame_time: Default::default(),
            frames: 0,
            last_print: Instant::now(),
            start_frame: Instant::now()
        }
    }
    
    pub fn start_frame(&mut self) {
        self.start_frame = Instant::now();
    }

    pub fn count_draw_call(&mut self) {
        self.draw_calls += 1;
    }

    pub fn end_frame(&mut self, ) {
        self.frames += 1;
        self.frame_time += self.start_frame.elapsed();

        if self.last_print.elapsed().as_millis() > 1000 {
            info!(
                "({}fps@{}ms {} Draw call(s))",
	            self.frames,
                ((self.frame_time.as_micros() / self.frames as u128) as f32) / 1000.0,
                self.draw_calls / self.frames
            );
            self.draw_calls = 0;
            self.frames = 0;
	        self.frame_time = Duration::ZERO;
	        self.last_print = Instant::now();
        }
    }
}
