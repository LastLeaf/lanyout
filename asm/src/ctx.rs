use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Ctx<T> {
    content: Arc<Mutex<T>>
}

impl<T> Ctx<T> {
    pub fn context<F>(&mut self, f: F) where F: Fn(&mut T) {
        f(&mut *self.content.lock().unwrap())
    }
    pub fn get(&mut self) -> MutexGuard<T> {
        self.content.lock().unwrap()
    }
}
