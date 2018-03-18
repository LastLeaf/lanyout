#![macro_use]

use std::sync::{Arc, Mutex};
use std::fmt;

use super::CanvasContext;

pub trait ElementContent: Send + fmt::Debug {
    fn name(&self) -> &'static str;
    fn draw(&self, ctx: &CanvasContext, element: &Element);
}

#[derive(Debug)]
pub struct Element {
    pub children: Vec<Arc<Mutex<Element>>>,
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
            child.lock().unwrap().draw(ctx);
        });
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}", name = self.content.name())
    }
}

#[derive(Debug)]
pub struct EmptyElement {}

impl EmptyElement {
    pub fn new() -> Self {
        EmptyElement {}
    }
}

impl ElementContent for EmptyElement {
    fn name(&self) -> &'static str {
        "EmptyElement"
    }
    fn draw(&self, _ctx: &CanvasContext, _element: &Element) {
        // do nothing
        // println!("Attempted to draw an EmptyElement");
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
        let mut temp_element_child = element! ( $e { $($c)* });
        $v.children.push(temp_element_child);
        __element_children! ($v, $($r)*);
    }
}

#[macro_export]
macro_rules! element {
    ($e:ident) => {
        element! ($e {})
    };
    ($e:ident { $($c:tt)* }) => {{
        let mut temp_element = Arc::new(Mutex::new(Element::new(Box::new($e::new()))));
        {
            let mut _temp_element_inner = temp_element.lock().unwrap();
            __element_children! (_temp_element_inner, $($c)*);
        }
        temp_element
    }}
}

pub mod test {
    use super::{Element, EmptyElement};
    use std::sync::{Arc, Mutex};

    pub fn test() -> i32 {
        let _elem = element! {
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
