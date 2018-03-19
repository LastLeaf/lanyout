#![macro_use]

pub struct Obj<T> {
    content: Arc<Mutex<T>>
}

impl<T> Deref for Obj<T> {
    type Target = T;
    fn deref_mut(&mut self) -> &Self::Target {
        &*self.content.lock().unwrap()
    }
}

impl<T> DerefMut for Obj<T> {
    type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.content.lock().unwrap()
    }
}
