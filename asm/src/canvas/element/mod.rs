#![macro_use]

mod empty_element;
mod image;
pub type EmptyElement = empty_element::EmptyElement;
pub type Image = image::Image;

use std::fmt;
use std::any::Any;
use super::super::ctx::Ctx;
use super::CanvasContext;

pub trait ElementContent: Any + Send + fmt::Debug {
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
    pub fn get_content_mut<T: 'static>(&mut self) -> &mut T where T: Sized {
        (*self.content).as_any_mut().downcast_mut::<T>().unwrap()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}", name = self.content.name())
    }
}

macro_rules! __element_children {
    ($v:ident, $t:ident, ) => {};
    ($v:ident, $t:ident, $k:ident = $a:expr; $($r:tt)*) => {
        $v.$k = $a;
        __element_children! ($v, $t, $($r)*);
    };
    ($v:ident, $t:ident, . $k:ident = $a:expr; $($r:tt)*) => {
        $v.get_content_mut::<$t>().$k = $a;
        __element_children! ($v, $t, $($r)*);
    };
    ($v:ident, $t:ident, . $k:ident ( $($a:expr),* ); $($r:tt)*) => {
        $v.get_content_mut::<$t>().$k($($a),*);
        __element_children! ($v, $t, $($r)*);
    };
    ($v:ident, $t:ident, $e:ident; $($r:tt)*) => {
        __element_children! ($v, $t, $e {}; $($r)*);
    };
    ($v:ident, $t:ident, $e:ident { $($c:tt)* }; $($r:tt)*) => {
        let mut temp_element_child = element_tree! ( $e { $($c)* });
        $v.children.push(temp_element_child);
        __element_children! ($v, $t, $($r)*);
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
            __element_children! (_temp_element_inner, $e, $($c)*);
        }
        temp_element
    }}
}

pub mod test {
    use super::{Element, EmptyElement, Image};
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
                Image {
                    .load("https://avatars0.githubusercontent.com/u/2016597?s=460&v=4");
                };
            }
        };
        // println!("{}", elem.lock().unwrap().name());
        return 0;
    }
}
