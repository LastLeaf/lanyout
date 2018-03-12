use std::sync::{Arc, Mutex};
use super::frame;

pub enum Animation {
    Timing(Box<TimingAnimation>),
    FrameCount(Box<FrameCountAnimation>)
}

pub trait TimingAnimation: Send + Sync {
    fn frame(&mut self, current_time: f64, total_time: f64);
    fn end(&mut self, total_time: f64);
}

pub trait FrameCountAnimation: Send + Sync {
    fn frame(&mut self, current_frame: i32, total_frames: i32);
    fn end(&mut self, total_frames: i32);
}

pub struct LinearTimingAnimation {
    start_value: f64,
    end_value: f64,
    current_value: f64
}

impl LinearTimingAnimation {
    pub fn new(start_value: f64, end_value: f64) -> Self {
        return LinearTimingAnimation {
            start_value,
            end_value,
            current_value: 0.
        };
    }
}

impl TimingAnimation for LinearTimingAnimation {
    fn frame(&mut self, current_time: f64, total_time: f64) {
        self.current_value = current_time / total_time * (self.end_value - self.start_value) + self.start_value;
        println!("LinearTimingAnimation frame {}", self.current_value);
    }
    fn end(&mut self, _total_time: f64) {
        self.current_value = self.end_value;
        println!("LinearTimingAnimation end {}", self.current_value);
    }
}

pub struct AnimationObject {
    start_time: f64,
    total_time: f64,
    current_frame: i32,
    total_frames: i32,
    animation: Animation
}

impl AnimationObject {
    pub fn new(ani: Animation) -> Self {
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
        frame::bind(Arc::new(Mutex::new(self)));
        // TODO unbind
    }
    pub fn exec_frames(mut self, total_frames: i32) {
        self.total_frames = total_frames;
        frame::bind(Arc::new(Mutex::new(self)));
    }
}

impl frame::Frame for AnimationObject {
    fn frame(&mut self, timestamp: f64) {
        match self.animation {
            Animation::Timing(ref mut x) => {
                if self.total_time <= timestamp - self.start_time {
                    x.end(self.total_time)
                } else {
                    x.frame(timestamp - self.start_time, self.total_time);
                }
            },
            Animation::FrameCount(ref mut x) => {
                if self.current_frame >= self.total_frames {
                    x.end(self.total_frames);
                } else {
                    x.frame(self.current_frame, self.total_frames);
                }
            }
        }
        self.current_frame += 1;
    }
}

pub mod test {
    use super::{Animation, AnimationObject, LinearTimingAnimation};

    pub fn test() -> i32 {
        AnimationObject::new(Animation::Timing(Box::new(LinearTimingAnimation::new(0., 100.)))).exec_time(3000.);
        return 0;
    }
}
