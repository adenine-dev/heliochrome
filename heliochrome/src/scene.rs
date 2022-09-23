use crate::{
    camera::Camera,
    color::Color,
    image::Image,
    maths::{vec2, vec3},
    object::Object,
};

pub enum SkyBox {
    Color(Color),
    Equirectangular(Image),
}

impl SkyBox {
    pub fn sample(&self, dir: vec3) -> Color {
        match self {
            SkyBox::Color(c) => *c,
            SkyBox::Equirectangular(i) => {
                let uv = vec2::new(
                    0.5 + dir.z.atan2(dir.x) / std::f32::consts::TAU,
                    0.5 + dir.y.asin() / std::f32::consts::PI,
                );
                i.sample_uv(&uv)
            }
        }
    }
}
pub struct Scene {
    pub camera: Camera,
    pub skybox: SkyBox,

    pub objects: Vec<Object>,
}
impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox) -> Self {
        Self {
            camera,
            skybox,
            objects: vec![],
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
