#![macro_use]

use std::sync::{Arc, Mutex, MutexGuard};
use std::any::{Any};

pub struct Ctx<T: Any> {
    content: Arc<Mutex<T>>
}

impl<T: Any> Clone for Ctx<T> {
    fn clone(&self) -> Self {
        Ctx {
            content: self.content.clone()
        }
    }
}

impl<T: Any> Ctx<T> {
    pub fn new(c: T) -> Self where T: Sized {
        Ctx {
            content: (Arc::new(Mutex::new(c)))
        }
    }
    pub fn ctx<F>(&mut self, f: F) where F: Fn(&mut T) {
        match self {
            &mut Ctx::Content(x) => {
                f(&mut *x.lock().unwrap())
            }
        }
    }
    pub fn get(&mut self) -> MutexGuard<T> {
        match self {
            &mut Ctx::Content(x) => {
                x.lock().unwrap()
            }
        }
    }
    pub fn ptr_eq(ctx1: &Ctx<T>, ctx2: &Ctx<T>) -> bool {
        Arc::ptr_eq(&ctx1.content, &ctx2.content)
    }
}

#[macro_export]
macro_rules! ctx {
    ($x:expr) => {
        Ctx::Content(Arc::new(Mutex::new($x)))
    }
}

// test

struct Implementor ();
impl super::frame::Frame for Implementor {
    fn frame(&mut self, _timestamp: f64) -> bool {
        return false
    }
}
fn test() {
    super::frame::bind(Ctx::new(Implementor{}));
}
fn test2() {
    super::frame::bind(ctx!(Implementor{}));
}
