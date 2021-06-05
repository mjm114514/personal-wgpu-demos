use std::time::Duration;
use std::time::Instant;

pub struct Timer{
    base_time: Instant,
    paused_duration: Duration,
    stop_time: Instant,
    delta_time: Duration,
    curr_time: Instant,
    stopped: bool
}

impl Timer{
    pub fn new() -> Self{
        Self{
            base_time: Instant::now(),
            paused_duration: Duration::from_nanos(0),
            stop_time: Instant::now(),
            delta_time: Duration::from_nanos(0),
            curr_time: Instant::now(),
            stopped: false
        }
    }

    // Returns the total time elapsed since reset() was called
    // NOT counting any time when the clock is stopped.
    pub fn total_time(&self) -> f32{
        if self.stopped {
            return (self.stop_time.duration_since(self.base_time) - self.paused_duration).as_secs_f32();
        }
        else{
            return (self.curr_time.duration_since(self.base_time) - self.paused_duration).as_secs_f32();
        }
    }

    pub fn delta_time(&self) -> f32{
        self.delta_time.as_secs_f32()
    }

    pub fn reset(&mut self){
        let curr_time = Instant::now();

        self.base_time = curr_time;
        self.curr_time = curr_time;
        self.stopped = false;
    }

    pub fn start(&mut self){
        if self.stopped {
            // Accumulate the time between stop and start pairs.
            let curr_time = Instant::now();
            self.paused_duration += curr_time.duration_since(self.stop_time);
            self.curr_time = curr_time;
            self.stopped = false;
        }
    }

    pub fn stop(&mut self){
        if !self.stopped {
            self.stop_time = Instant::now();
            self.stopped = true;
        }
    }

    pub fn tick(&mut self){
        if self.stopped {
            self.delta_time = Duration::from_nanos(0);
            return;
        }
        let curr_time = Instant::now();
        self.delta_time = curr_time.duration_since(self.curr_time);
        self.curr_time = curr_time;
    }
}