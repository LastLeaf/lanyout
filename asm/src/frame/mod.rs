use super::ctx::Ctx;

pub mod animation;

pub trait Frame: Send {
    fn frame(&mut self, timestamp: f64) -> bool;
}

lazy_static! {
    static ref FRAME_OBJECTS: Ctx<Vec<Ctx<Frame>>> = Ctx::new(vec![]);
}

// static FRAME_OBJECTS: *mut Ctx<Vec<Ctx<Frame>>> = 0 as *mut Ctx<Vec<Ctx<Frame>>>;

pub fn bind(fo: Ctx<Frame>) {
    let mut frame_objects = Box::from_raw(FRAME_OBJECTS).get();
    if frame_objects.len() == 0 {
        lib!(enable_animation_frame());
    }
    frame_objects.push(fo.clone());
}

pub fn unbind(fo: Ctx<Frame>) -> bool {
    let mut frame_objects = Box::from_raw(FRAME_OBJECTS).get();
    return match frame_objects.iter().position(|ref x| Ctx::ptr_eq(&x, &fo)) {
        None => false,
        Some(index) => {
            frame_objects.remove(index);
            if frame_objects.len() == 0 {
                lib!(disable_animation_frame());
            }
            return true;
        }
    };
}

pub fn generate(timestamp: f64) {
    Box::from_raw(FRAME_OBJECTS).get().iter_mut().for_each(|x| {
        let ret = x.get().frame(timestamp);
        if ret == false {
            unbind(x.clone());
        }
    });
}

pub fn init() {
    // FRAME_OBJECTS = Box::into_raw(Box::new(Ctx::new(vec![])));
}

pub mod test {
    pub fn test() -> i32 {
        let mut err = 0;
        err += super::animation::test::test();
        return err;
    }
}
