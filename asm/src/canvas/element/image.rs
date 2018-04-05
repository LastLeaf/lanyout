use std::ffi::CString;
use super::super::CanvasContext;
use super::Element;

#[derive(Debug)]
pub struct Image {
    canvas_index: i32,
    loader: Option<ImageLoader>
}

impl Image {
    pub fn new(ctx: &mut CanvasContext) -> Self {
        Image {
            canvas_index: ctx.index,
            loader: Some(ImageLoader::new(ctx))
        }
    }
    pub fn load<T: Into<Vec<u8>>>(&mut self, url: T) {
        self.loader.take().unwrap().load(url);
    }
}

impl super::ElementContent for Image {
    fn name(&self) -> &'static str {
        "Image"
    }
    fn draw(&self, _element: &Element) {
        // do nothing
        // println!("Attempted to draw an EmptyElement");
    }
}

#[derive(Debug)]
pub struct ImageLoader {
    canvas_index: i32,
    id: i32
}

impl ImageLoader {
    pub fn new(ctx: &mut CanvasContext) -> Self {
        ImageLoader {
            canvas_index: ctx.index,
            id: ctx.alloc_image_id()
        }
    }
    pub fn load<T: Into<Vec<u8>>>(self, url: T) {
        lib!(image_load_url(self.canvas_index, self.id, CString::new(url).unwrap().into_raw(), lib_callback!(self)));
    }
}

lib_define_callback! (ImageLoader {
    fn callback(&mut self, ret_code: i32) {
        println!("Image load result: {} for {} {}", ret_code, self.canvas_index, self.id);
    }
});

impl Drop for ImageLoader {
    fn drop(&mut self) {
        lib!(image_unload(self.canvas_index, self.id));
    }
}
