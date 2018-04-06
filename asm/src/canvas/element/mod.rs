#![macro_use]

use downcast_rs::Downcast;

mod empty_element;
mod image;
pub type EmptyElement = empty_element::EmptyElement;
pub type Image = image::Image;

use std::fmt;
use super::super::ctx::Ctx;
use super::CanvasContext;

pub trait ElementContent: Downcast + Send + fmt::Debug {
    fn name(&self) -> &'static str;
    fn draw(&self, element: &Element);
}

impl_downcast!(ElementContent);

pub struct Element {
    pub children: Vec<Ctx<Element>>,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    content: Box<ElementContent>
}

impl Element {
    pub fn new(_ctx: &mut CanvasContext, content: Box<ElementContent>) -> Self {
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
    pub fn draw(&self) {
        self.content.draw(self);
        self.children.iter().for_each(|child| {
            child.get().draw();
        });
    }
    pub fn get_content_ref<T: ElementContent>(&self) -> &T {
        self.content.downcast_ref::<T>().unwrap()
    }
    pub fn get_content_mut<T: ElementContent>(&mut self) -> &mut T {
        self.content.downcast_mut::<T>().unwrap()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}", name = self.content.name())
    }
}

macro_rules! __element_children {
    ($ctx:expr, $v:ident, $t:ident, ) => {};
    ($ctx:expr, $v:ident, $t:ident, $k:ident = $a:expr; $($r:tt)*) => {
        $v.$k = $a;
        __element_children! ($ctx, $v, $t, $($r)*);
    };
    ($ctx:expr, $v:ident, $t:ident, . $k:ident = $a:expr; $($r:tt)*) => {
        $v.get_content_mut::<$t>().$k = $a;
        __element_children! ($ctx, $v, $t, $($r)*);
    };
    ($ctx:expr, $v:ident, $t:ident, . $k:ident ( $($a:expr),* ); $($r:tt)*) => {
        $v.get_content_mut::<$t>().$k($($a),*);
        __element_children! ($ctx, $v, $t, $($r)*);
    };
    ($ctx:expr, $v:ident, $t:ident, $e:ident; $($r:tt)*) => {
        __element_children! ($ctx, $v, $t, $e {}; $($r)*);
    };
    ($ctx:expr, $v:ident, $t:ident, $e:ident { $($c:tt)* }; $($r:tt)*) => {
        let mut temp_element_child = __element_tree! ( $ctx, $e { $($c)* });
        $v.children.push(temp_element_child);
        __element_children! ($ctx, $v, $t, $($r)*);
    }
}

macro_rules! __element_tree {
    ($ctx:expr, $e:ident) => {
        __element_tree! ($ctx, $e {})
    };
    ($ctx:expr, $e:ident { $($c:tt)* }) => {{
        let mut temp_content = Box::new($e::new($ctx));
        let mut temp_element = Ctx::new(Element::new($ctx, temp_content));
        {
            let mut _temp_element_inner = temp_element.get();
            __element_children! ($ctx, _temp_element_inner, $e, $($c)*);
        }
        temp_element
    }}
}

#[macro_export]
macro_rules! element {
    ([$ctx:expr] $($c:tt)*) => {{
        __element_tree! ($ctx, $($c)*)
    }}
}

pub mod test {
    use super::{Element, EmptyElement, Image};
    use super::super::super::ctx::Ctx;
    use super::super::Canvas;

    pub fn test() -> i32 {
        let canvas = Canvas::new(1);
        let ctx = canvas.get_context();
        let ctx_mut = &mut *ctx.get();
        let elem = element! {
             [ctx_mut] EmptyElement {
                left = 10.;
                top = 20.;
                EmptyElement;
                EmptyElement {
                    EmptyElement;
                    top = 20.;
                };
                Image {
                    width = 100.;
                    height = 100.;
                    .load("../resources/lastleaf.png");
                };
            }
        };
        let ctx = canvas.get_context();
        let root_elem = ctx.get().get_root_element();
        root_elem.get().children.push(elem);
        return 0;
    }
}
