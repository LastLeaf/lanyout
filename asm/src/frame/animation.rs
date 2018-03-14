use std::sync::{Arc, Mutex};

pub trait Animation: Send {
    fn frame(&mut self, current_frame: i32, total_frames: i32, current_time: f64, total_time: f64);
    fn end(&mut self, total_frames: i32, total_time: f64);
}

pub struct LinearTimingAnimation<T: Send> {
    data: T,
    start_value: f64,
    end_value: f64,
    current_value: f64
}

impl<T: Send> LinearTimingAnimation<T> {
    pub fn new(data: T, start_value: f64, end_value: f64) -> Self {
        return LinearTimingAnimation {
            data,
            start_value,
            end_value,
            current_value: 0.
        };
    }
}

impl<T: Send> Animation for LinearTimingAnimation<T> {
    fn frame(&mut self, _current_frame: i32, _total_frames: i32, current_time: f64, total_time: f64) {
        self.current_value = current_time / total_time * (self.end_value - self.start_value) + self.start_value;
        println!("LinearTimingAnimation frame {}", self.current_value);
    }
    fn end(&mut self, _total_frames: i32, _total_time: f64) {
        self.current_value = self.end_value;
        println!("LinearTimingAnimation end {}", self.current_value);
    }
}

pub struct AnimationObject {
    start_time: f64,
    total_time: f64,
    current_frame: i32,
    total_frames: i32,
    animation: Box<Animation>
}

impl AnimationObject {
    pub fn new(ani: Box<Animation>) -> Self {
        return AnimationObject {
            start_time: 0.,
            total_time: 0.,
            current_frame: 0,
            total_frames: 0,
            animation: ani,
        }
    }
    pub fn exec_time(mut self, total_time: f64) {
        self.total_time = total_time;
        super::bind(Arc::new(Mutex::new(self)));
        // TODO unbind
    }
    pub fn exec_frames(mut self, total_frames: i32) {
        self.total_frames = total_frames;
        super::bind(Arc::new(Mutex::new(self)));
    }
}

impl super::Frame for AnimationObject {
    fn frame(&mut self, timestamp: f64) -> bool {
        if self.total_time <= timestamp - self.start_time && self.current_frame >= self.total_frames {
            self.animation.end(self.total_frames, self.total_time);
            return false;
        }
        self.animation.frame(self.current_frame, self.total_frames, timestamp - self.start_time, self.total_time);
        self.current_frame += 1;
        return true;
    }
}

pub mod test {
    use super::{AnimationObject, LinearTimingAnimation};

    pub fn test() -> i32 {
        AnimationObject::new(Box::new(LinearTimingAnimation::new(0, 0., 100.))).exec_time(3000.);
        return 0;
    }
}
