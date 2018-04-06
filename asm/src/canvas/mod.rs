use super::ctx::Ctx;
use super::frame;

mod element;

pub type Element = element::Element;
pub type EmptyElement = element::EmptyElement;

#[derive(Clone)]
pub struct CanvasContext {
    index: i32,
    tex_size: i32,
    tex_count: i32,
    tex_max_draws: i32,
    image_id_inc: i32,
    root_element: Option<Ctx<Element>>,
}

#[derive(Clone)]
pub struct Canvas {
    context: Ctx<CanvasContext>
}

impl Canvas {
    pub fn new(index: i32) -> Self {
        lib!(bind_canvas(index));
        let context = Ctx::new(CanvasContext {
            index,
            tex_size: lib!(tex_get_size(index)) as i32,
            tex_count: lib!(tex_get_count(index)) as i32,
            tex_max_draws: lib!(tex_get_max_draws()) as i32,
            image_id_inc: 1,
            root_element: None,
        });
        context.ctx(|x| {
            println!("Canvas binded: tex_size {}; tex_count {}; tex_max_draws {}", x.tex_size, x.tex_count, x.tex_max_draws);
            let root_element = Some(element! {
                [x] EmptyElement
            });
            x.root_element = root_element;
        });
        frame::bind(context.clone());
        return Canvas {
            context
        };
    }
    pub fn destroy(&mut self) {
        frame::unbind(self.context.clone());
    }
    pub fn get_context(&self) -> Ctx<CanvasContext> {
        return self.context.clone();
    }
    pub fn context<F>(&mut self, f: F) where F: Fn(&mut CanvasContext) {
        f(&mut *self.context.get());
    }
}

impl Drop for CanvasContext {
    fn drop(&mut self) {
        lib!(unbind_canvas(self.index));
    }
}

impl frame::Frame for CanvasContext {
    fn frame(&mut self, _timestamp: f64) -> bool {
        self.clear();
        let root_element = self.root_element.as_mut().unwrap();
        root_element.get().draw();
        return true;
    }
}

impl CanvasContext {
    pub fn set_canvas_size(&mut self, w: i32, h: i32) {
        lib!(set_canvas_size(self.index, w, h));
    }
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        lib!(set_clear_color(self.index, r, g, b, a));
    }
    pub fn clear(&mut self) {
        lib!(clear(self.index));
    }
    pub fn get_root_element(&mut self) -> Ctx<Element> {
        self.root_element.as_mut().unwrap().clone()
    }

    pub fn alloc_image_id(&mut self) -> i32 {
        let ret = self.image_id_inc + 1;
        self.image_id_inc += 1;
        ret
    }
}

pub mod test {
    use super::super::ctx::Ctx;
    use super::Canvas;
    use super::super::frame::animation::{TimingAnimation, AnimationObject, LinearTiming};

    pub fn test() -> i32 {
        let mut canvas = Canvas::new(0);

        canvas.context(|ctx| {
            ctx.set_canvas_size(400, 300);
        });

        struct BackgroundColorAni(Canvas);
        impl TimingAnimation for BackgroundColorAni {
            fn progress(&mut self, current_value: f64, _current_time: f64, _total_time: f64) {
                self.0.context(|ctx| {
                    ctx.set_clear_color(0., current_value as f32, current_value as f32, 1.);
                })
            }
        }

        let mut ani_obj = Ctx::new(AnimationObject::new(Ctx::new(LinearTiming::new(BackgroundColorAni(canvas.clone()), 0., 1.))));
        AnimationObject::exec(&mut ani_obj, 0, 3000.);

        let mut err = 0;
        err += super::element::test::test();
        return err;
    }
}
