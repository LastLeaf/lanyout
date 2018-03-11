use std::sync::{Arc, Mutex};

pub trait Frame: Send + Sync {
    fn frame(&mut self, timestamp: f64);
}

type ArcFrame = Arc<Mutex<Frame>>;

lazy_static! {
    static ref FRAME_OBJECTS: Arc<Mutex<Vec<ArcFrame>>> = Arc::new(Mutex::new(vec![]));
}

pub fn bind_frame(fo: Arc<Mutex<Frame>>) {
    FRAME_OBJECTS.lock().unwrap().push(fo);
}

pub fn unbind_frame(fo: Arc<Mutex<Frame>>) -> bool {
    let mut frame_objects = FRAME_OBJECTS.lock().unwrap();
    return match frame_objects.iter().position(|ref x| Arc::ptr_eq(&x, &fo)) {
        None => false,
        Some(index) => {
            frame_objects.remove(index);
            return true;
        }
    };
}

pub fn generate_frame(timestamp: f64) {
    FRAME_OBJECTS.lock().unwrap().iter_mut().for_each(|x| {
        x.lock().unwrap().frame(timestamp)
    });
}
