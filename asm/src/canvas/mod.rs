use super::lib_interfaces;

pub struct Canvas {
    index: i32
}

impl Canvas {
    pub fn new(index: i32) -> Self {
        lib!(bind_canvas(index));
        return Canvas {
            index
        };
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        lib!(unbind_canvas(self.index));
    }
}

impl Canvas {
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        lib!(set_clear_color(self.index, r, g, b, a));
    }
    pub fn clear(&mut self) {
        lib!(clear(self.index));
    }
}
