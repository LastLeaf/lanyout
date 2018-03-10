use std::sync::{Arc, Mutex};

lazy_static! {
    static ref FRAME_OBJECTS: Mutex<Vec<Arc<FrameObject>>> = Mutex::new(vec![]);
}

trait FrameObject {
    fn frame(&mut self, timestamp: f64) -> ();
}

pub fn bind_frame(&mut fo: FrameObject) {
    FRAME_OBJECTS.lock().unwrap().push(fo);
}

pub fn unbind_frame(&mut fo: FrameObject) -> bool {
    let mut frameObjects = FRAME_OBJECTS.lock().unwrap();
    let index = match frameObjects.iter().position(|&x| x == fo) {
        None => -1,
        Some(index) => index
    };
    if index >= 0 {
        frameObjects.remove(index);
        return true;
    }
    return false;
}

pub fn generate_frame(timestamp: f64) {
    FRAME_OBJECTS.lock().unwrap().iter_mut().for_each(|&mut x| x.frame(timestamp));
    println!("Frame generated!");
}
