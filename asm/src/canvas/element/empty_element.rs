use super::super::CanvasContext;
use super::Element;

#[derive(Debug)]
pub struct EmptyElement {}

impl EmptyElement {
    pub fn new() -> Self {
        EmptyElement {}
    }
}

impl super::ElementContent for EmptyElement {
    fn name(&self) -> &'static str {
        "EmptyElement"
    }
    fn draw(&self, _ctx: &CanvasContext, _element: &Element) {
        // do nothing
        // println!("Attempted to draw an EmptyElement");
    }
}
