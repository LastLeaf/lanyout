#![macro_use]

mod empty_element;
mod image;
pub type EmptyElement = empty_element::EmptyElement;
pub type Image = image::Image;

use std::fmt;
use super::super::ctx::Ctx;
use super::CanvasContext;

pub trait ElementContent: Send + fmt::Debug {
    fn name(&self) -> &'static str;
    fn draw(&self, ctx: &CanvasContext, element: &Element);
}

pub struct Element {
    pub children: Vec<Ctx<Element>>,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    content: Box<ElementContent>
}

impl Element {
    pub fn new(content: Box<ElementContent>) -> Self {
        Element {
            children: vec![],
            left: 0.,
            top: 0.,
            width: 0.,
            height: 0.,
            content
        }
    }
    pub fn name(&self) -> &'static str {
        self.content.name()
    }
    pub fn draw(&self, ctx: &CanvasContext) {
        self.content.draw(ctx, self);
        self.children.iter().for_each(|child| {
            child.get().draw(ctx);
        });
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}", name = self.content.name())
    }
}

macro_rules! __element_children {
    ($v:ident, ) => {};
    ($v:ident, $k:ident = $a:expr; $($r:tt)*) => {
        $v.$k = $a;
        __element_children! ($v, $($r)*);
    };
    ($v:ident, $e:ident; $($r:tt)*) => {
        __element_children! ($v, $e {}; $($r)*);
    };
    ($v:ident, $e:ident { $($c:tt)* }; $($r:tt)*) => {
        let mut temp_element_child = element_tree! ( $e { $($c)* });
        $v.children.push(temp_element_child);
        __element_children! ($v, $($r)*);
    }
}

#[macro_export]
macro_rules! element_tree {
    ($e:ident) => {
        element_tree! ($e {})
    };
    ($e:ident { $($c:tt)* }) => {{
        let mut temp_element = Ctx::new(Element::new(Box::new($e::new())));
        {
            let mut _temp_element_inner = temp_element.get();
            __element_children! (_temp_element_inner, $($c)*);
        }
        temp_element
    }}
}

pub mod test {
    use super::{Element, EmptyElement};
    use super::super::super::ctx::Ctx;

    pub fn test() -> i32 {
        let _elem = element_tree! {
            EmptyElement {
                left = 10.;
                top = 20.;
                EmptyElement;
                EmptyElement {
                    EmptyElement;
                    top = 20.;
                };
            }
        };
        // println!("{}", elem.lock().unwrap().name());
        return 0;
    }
}
