use std::ffi::CString;
use super::super::CanvasContext;
use super::Element;

#[derive(Debug)]
pub struct Image {}

impl Image {
    pub fn new() -> Self {
        Image {}
    }
    pub fn load<T: Into<Vec<u8>>>(&mut self, url: T) {
        // TODO impl
        lib!(image_load_url(0, 0, CString::new(url).unwrap().into_raw(), lib_callback!(ImageLoadInfo {
            canvas_index: 0,
            id: 0,
        })));
    }
}

impl super::ElementContent for Image {
    fn name(&self) -> &'static str {
        "Image"
    }
    fn draw(&self, _ctx: &CanvasContext, _element: &Element) {
        // do nothing
        // println!("Attempted to draw an EmptyElement");
    }
}

struct ImageLoadInfo {
    canvas_index: i32,
    id: i32,
}

lib_define_callback! (ImageLoadInfo {
    fn callback(&mut self, ret_code: i32) {
        println!("Image load result: {}", ret_code);
    }
});
