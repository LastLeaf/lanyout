#![macro_use]

use std::sync::{Arc, Mutex};
use std::fmt;

pub trait ElementContent: Send + fmt::Debug {
    fn name(&self) -> &'static str;
    fn draw(&self, element: &Element);
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
    pub fn draw(&self) {
        self.content.draw(self);
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
    fn draw(&self, _element: &Element) {
        // do nothing
    }
}

#[macro_export]
macro_rules! __element_children {
    ($v:ident,) => {};
    ($v:ident, $e:ident ( $($a:tt)* ) { $($c:tt)* } $($r:tt)*) => {
        $v.children.push(element! { $e ( $($a)* ) { $($c)* } });
        __element_children! ($v, $($r)*)
    }
}

#[macro_export]
macro_rules! element {
    ($e:ident ( $($k:ident = $v:expr),* ) { $($c:tt)* }) => {
        {
            let elem_arc = Arc::new(Mutex::new(Element::new(Box::new($e::new()))));
            {
                let element = &mut *elem_arc.lock().unwrap();
                $(
                    element.$k = $v;
                ),*
                __element_children! (element, $($c)*)
            }
            elem_arc
        }
    }
}

pub mod test {
    use super::{Element, EmptyElement};
    use std::sync::{Arc, Mutex};

    pub fn test() -> i32 {
        let elem = element! {
            EmptyElement() {
                EmptyElement() {}
                EmptyElement() {}
                EmptyElement() {}
            }
        };
        println!("{}", elem.lock().unwrap());
        return 0;
    }
}
