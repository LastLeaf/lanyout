use super::super::CanvasContext;
use super::Element;

#[derive(Debug)]
pub struct Image {}

impl Image {
    pub fn new() -> Self {
        Image {}
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
