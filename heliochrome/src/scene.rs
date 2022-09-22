use crate::{camera::Camera, image::Image, object::Object};

pub struct Scene {
    pub camera: Camera,
    pub skybox: Option<Image>,

    pub objects: Vec<Object>,
}
impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            skybox: None,
            objects: vec![],
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
