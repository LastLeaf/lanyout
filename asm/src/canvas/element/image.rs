use std::ffi::CString;
use super::super::CanvasContext;
use super::Element;

#[derive(Debug)]
pub struct Image {
    canvas_index: i32,
    image_id: i32,
    loader: Option<ImageLoader>
}

impl Image {
    pub fn new(ctx: &mut CanvasContext) -> Self {
        let loader = ImageLoader::new(ctx);
        let image_id = loader.get_id();
        Image {
            canvas_index: ctx.index,
            image_id,
            loader: Some(loader)
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
    fn draw(&self, elem: &Element) {
        // do nothing
        // println!("Attempted to draw an EmptyElement");
        if self.loader.is_some() {
            return
        }
        lib!(tex_set_image(self.canvas_index, self.image_id, self.image_id, elem.left, elem.top, elem.width, elem.height));
        lib!(tex_draw(self.canvas_index, 0, self.image_id, elem.left, elem.top, elem.width, elem.height, elem.left, elem.top, elem.width, elem.height));
        lib!(tex_draw_end(self.canvas_index));
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
    pub fn get_id(&self) -> i32 {
        self.id
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
