const WIDTH: u16 = 400;
const HEIGHT: u16 = 225;

use crate::heliochrome::{context::Context, *};

pub fn make_context() -> Context {
    let context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 0.0),
            WIDTH as f32 / HEIGHT as f32,
        ),
    );

    context
}
