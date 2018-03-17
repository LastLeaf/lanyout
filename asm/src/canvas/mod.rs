use std::sync::{Arc, Mutex};
use super::frame;

mod element;

pub type Element = element::Element;
pub type EmptyElement = element::EmptyElement;

pub struct CanvasContext {
    index: i32
}

#[derive(Clone)]
pub struct Canvas {
    arc_ctx: Arc<Mutex<CanvasContext>>
}

impl Canvas {
    pub fn new(index: i32) -> Self {
        lib!(bind_canvas(index));
        let arc_ctx = Arc::new(Mutex::new(CanvasContext {
            index
        }));
        frame::bind(arc_ctx.clone());
        return Canvas {
            arc_ctx
        };
    }
    pub fn get_context_mutex(&self) -> Arc<Mutex<CanvasContext>> {
        return self.arc_ctx.clone();
    }
    pub fn context<F>(&mut self, f: F) where F: Fn(&mut CanvasContext) {
        f(&mut *self.arc_ctx.lock().unwrap());
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        frame::unbind(self.arc_ctx.clone());
    }
}

impl Drop for CanvasContext {
    fn drop(&mut self) {
        lib!(unbind_canvas(self.index));
    }
}

impl frame::Frame for CanvasContext {
    fn frame(&mut self, timestamp: f64) -> bool {
        // TODO
        println!("Update canvas: {}", timestamp);
        return true;
    }
}

impl CanvasContext {
    pub fn set_canvas_size(&mut self, w: i32, h: i32) {
        lib!(set_canvas_size(self.index, w, h));
    }
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        lib!(set_clear_color(self.index, r, g, b, a));
    }
    pub fn clear(&mut self) {
        lib!(clear(self.index));
    }
}

pub mod test {
    use super::Canvas;
    use super::super::frame::animation::{TimingAnimation, AnimationObject, LinearTiming};

    pub fn test() -> i32 {
        let mut canvas = Canvas::new(0);

        canvas.context(|ctx| {
            ctx.set_canvas_size(400, 300);
        });

        struct BackgroundColorAni(Canvas);
        impl TimingAnimation for BackgroundColorAni {
            fn progress(&mut self, current_value: f64, _current_time: f64, _total_time: f64) {
                self.0.context(|ctx| {
                    ctx.set_clear_color(0., current_value, current_value, 1.);
                    ctx.clear();
                })
            }
        }

        AnimationObject::new(Box::new(LinearTiming::new(BackgroundColorAni(canvas.clone()), 0., 1.))).exec(0, 3000.);

        let mut err = 0;
        err += super::element::test::test();
        return err;
    }
}
