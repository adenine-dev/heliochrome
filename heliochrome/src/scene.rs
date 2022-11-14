use std::{error::Error, path::Path};

use crate::{
    accel::{Accel, Accelerator},
    camera::Camera,
    color::Color,
    hittables::Intersection,
    image::Image,
    loader::{collect_until_next_item, parse_into, FromHCY},
    materials::Scatterable,
    maths::{vec2, vec3, Ray},
    object::Object,
    pdf::ObjectPdf,
};

#[derive(Debug)]
pub enum SkyBox {
    Color(Color),
    Equirectangular(Image),
    Debug,
}

impl SkyBox {
    pub fn sample(&self, dir: vec3) -> Color {
        match self {
            SkyBox::Color(c) => *c,
            SkyBox::Equirectangular(img) => {
                let uv = vec2::new(
                    0.5 + dir.z.atan2(dir.x) / std::f32::consts::TAU,
                    0.5 + dir.y.asin() / std::f32::consts::PI,
                );

                img.sample_uv(&uv)
            }
            SkyBox::Debug => ((dir.normalized() + vec3::splat(1.0)) * 0.5).into(),
        }
    }
}

impl FromHCY for SkyBox {
    fn from_hcy(member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        if let Some(member) = member {
            match member.trim() {
                "color" => {
                    let color = lines.iter().fold(
                        Err(Box::<dyn Error>::from("invalid color")),
                        |a, x| {
                            if a.is_err() {
                                let split = x.trim().split_once("color: ");
                                if let Some((_, color)) = split {
                                    parse_into::<Color>(color)
                                } else {
                                    a
                                }
                            } else {
                                a
                            }
                        },
                    )?;
                    Ok(SkyBox::Color(color))
                }
                "debug" => Ok(SkyBox::Debug),
                "hdri" => {
                    let path = lines.iter().fold(
                        Err(Box::<dyn Error>::from("invalid path syntax")),
                        |a, x| {
                            if a.is_err() {
                                let split = x.trim().split_once("path: ");
                                if let Some((_, path)) = split {
                                    Ok(Path::new(path))
                                } else {
                                    a
                                }
                            } else {
                                a
                            }
                        },
                    )?;
                    Ok(SkyBox::Equirectangular(Image::load_from_hdri(path)?))
                }
                _ => Err(format!("unknown member {member}"))?,
            }
        } else {
            Err("invalid member syntax")?
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub skybox: SkyBox,

    pub objects: Accel<Object>,
    pub important_indices: Vec<usize>,
}

impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox, objects: Vec<Object>) -> Self {
        let objects = Accel::new(objects);
        let important_indices = objects
            .hittables
            .iter()
            .enumerate()
            .filter_map(|(idx, obj)| {
                if !obj.material.is_important() {
                    None
                } else {
                    Some(idx)
                }
            })
            .collect::<Vec<_>>();

        Self {
            camera,
            skybox,
            objects,
            important_indices,
        }
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Intersection, &Object)> {
        self.objects.intersect_obj(ray, t_min, t_max)
    }

    pub fn make_importance_pdf(&self, origin: &vec3) -> Vec<ObjectPdf> {
        self.important_indices
            .iter()
            .map(|idx| ObjectPdf::new(self.objects.get_nth(*idx), *origin))
            .collect()
    }

    pub fn get_importants(&self) -> Vec<Object> {
        self.important_indices
            .iter()
            .map(|idx| self.objects.get_nth(*idx).clone())
            .collect()
    }
}

impl FromHCY for Scene {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut camera = None;
        let mut skybox = None;
        let mut objects = None;

        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "camera" => {
                    camera = Some(
                        Camera::from_hcy(None, collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse camera key: {err}"))?,
                    );
                }
                "skybox" => {
                    skybox = Some(
                        SkyBox::from_hcy(Some(value), collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse skybox key: {err}"))?,
                    )
                }
                "objects" => {
                    let mut objs: Vec<Object> = vec![];
                    let lines = collect_until_next_item(&mut line_iter);
                    let mut line_iter = lines.iter();
                    while let Some(line) = line_iter.next() {
                        let lines = collect_until_next_item(&mut line_iter);
                        if !lines.is_empty() {
                            objs.push(Object::from_hcy(Some(line), lines)?);
                        }
                    }
                    objects = Some(objs);
                }
                _ => {}
            }
        }
        Ok(Scene::new(
            camera.ok_or("Missing required key `camera`")?,
            skybox.ok_or("Missing required key `skybox`")?,
            objects.ok_or("Missing required key `objects`")?,
        ))
    }
}
