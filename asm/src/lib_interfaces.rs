#![macro_use]

extern {
    pub fn init_lib();
    pub fn enable_animation_frame();
    pub fn disable_animation_frame();
    pub fn bind_canvas(canvasIndex: i32);
    pub fn unbind_canvas(canvasIndex: i32);
    pub fn set_canvas_size(canvasIndex: i32, w: i32, h: i32);
    pub fn set_clear_color(canvasIndex: i32, r: f64, g: f64, b: f64, a: f64);
    pub fn clear(canvasIndex: i32);
    pub fn tex_get_size(canvasIndex: i32) -> i32;
    pub fn tex_get_count(canvasIndex: i32) -> i32;
    pub fn tex_get_max_draws() -> i32;
}

#[macro_export]
macro_rules! lib {
    ($x:ident($($y:expr),*)) => {
        unsafe {
            $crate::lib_interfaces::$x($($y),*)
        }
    }
}
