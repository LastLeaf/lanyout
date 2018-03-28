#![macro_use]

use std::os::raw::c_char;

extern {
    pub fn emscripten_exit_with_live_runtime();

    pub fn init_lib();
    pub fn enable_animation_frame();
    pub fn disable_animation_frame();

    pub fn bind_canvas(canvasIndex: i32);
    pub fn unbind_canvas(canvasIndex: i32);
    pub fn set_canvas_size(canvasIndex: i32, w: i32, h: i32);
    pub fn set_clear_color(canvasIndex: i32, r: f32, g: f32, b: f32, a: f32);
    pub fn clear(canvasIndex: i32);

    pub fn tex_get_size(canvasIndex: i32) -> i32;
    pub fn tex_get_count(canvasIndex: i32) -> i32;
    pub fn tex_get_max_draws() -> i32;
    pub fn tex_draw(canvasIndex: i32, drawIndex: i32, texIndex: i32, texX: f32, texY: f32, texW: f32, texH: f32, x: f32, y: f32, w: f32, h: f32);
    pub fn tex_draw_end(canvasIndex: i32);
    pub fn tex_set_text(canvasIndex: i32, id: i32, texIndex: i32, texX: f64, texY: f64, texWidth: f64, texHeight: f64);
    pub fn tex_set_image(canvasIndex: i32, id: i32, texIndex: i32, texX: f64, texY: f64, texWidth: f64, texHeight: f64);

    pub fn image_load_url(canvasIndex: i32, id: i32, url: *mut c_char, cbPtr: *mut Callback);

    pub fn text_bind_font_family(canvasIndex: i32, id: i32, fontFamily: *mut c_char);
    pub fn text_unbind_font_family(canvasIndex: i32, id: i32);
    pub fn text_set_font(canvasIndex: i32, fontSize: f32, fontFamilyId: i32);
    pub fn text_get_width(text: *mut c_char) -> f32;
    pub fn text_draw_in_canvas(text: *mut c_char, width: f32, fontSize: f32, fontFamilyId: i32);
    pub fn text_save_image_data(canvasIndex: i32, id: i32, x: f32, y: f32, width: f32, height: f32);
    pub fn text_remove_image_data(canvasIndex: i32, id: i32);
}

pub trait Callback {
    fn callback(&mut self, ret_code: i32);
}

pub fn register_callback(callback: Box<Callback>) -> *mut Callback {
    Box::into_raw(callback)
}

#[macro_export]
macro_rules! lib {
    ($x:ident($($y:expr),*)) => {
        unsafe {
            $crate::lib_interfaces::$x($($y),*)
        }
    }
}
