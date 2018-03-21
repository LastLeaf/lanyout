use std::sync::{Arc, Mutex, MutexGuard};

pub struct Ctx<T: ?Sized + 'static> {
    content: Arc<Mutex<T>>
}

impl<T: ?Sized> Clone for Ctx<T> {
    fn clone(&self) -> Self {
        Ctx {
            content: self.content.clone()
        }
    }
}

impl<T: ?Sized> Ctx<T> {
    pub fn new(c: T) -> Self where T: Sized + 'static {
        Ctx {
            content: Arc::new(Mutex::new(c))
        }
    }
    pub fn ctx<F>(&mut self, f: F) where F: Fn(&mut T) {
        f(&mut *self.content.lock().unwrap())
    }
    pub fn get(&mut self) -> MutexGuard<T> {
        self.content.lock().unwrap()
    }
    pub fn ptr_eq(ctx1: &Ctx<T>, ctx2: &Ctx<T>) -> bool {
        Arc::ptr_eq(&ctx1.content, &ctx2.content)
    }
}

// test

struct Implementor ();
impl super::frame::Frame for Implementor {
    fn frame(&mut self, _timestamp: f64) -> bool {
        return false
    }
}
fn test() -> Ctx<super::frame::Frame> {
    Ctx{
        content: Arc::new(Mutex::new(Implementor{}))
    }
}
fn test2() {
    super::frame::bind(&Ctx::new(Implementor{}));
}
