pub mod animation;

use std::sync::{Arc, Mutex};
use super::frame;

pub struct CanvasContext {
    index: i32
}

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
    fn frame(&mut self, timestamp: f64) {
        println!("Update canvas: {}", timestamp);
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

    pub fn test() -> i32 {
        let mut canvas = Canvas::new(0);
        Canvas::new(1);
        canvas.context(|ctx| {
            ctx.set_canvas_size(400, 300);
            ctx.set_clear_color(0., 1., 1., 0.5);
            ctx.clear();
        });
        return 0;
    }
}