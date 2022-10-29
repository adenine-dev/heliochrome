// TODO: more of this
// currently is pretty much an adaptation of https://github.com/rgl-epfl/brdf-loader

use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom, Write},
    os::windows::prelude::FileExt,
    path::{Path, PathBuf},
};

use super::{Scatter, Scatterable};
use crate::{
    hittables::Hit,
    maths::{vec2, vec3, Ray, Warp2D0, Warp2D2, Warp2D3, ONB},
    pdf::CosinePDF,
};

#[derive(Debug, PartialEq)]

pub enum TensorFieldType {
    Invalid,

    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,

    Float16,
    Float32,
    Float64,
}

impl From<u8> for TensorFieldType {
    fn from(x: u8) -> Self {
        match x {
            1 => TensorFieldType::UInt8,
            2 => TensorFieldType::Int8,
            3 => TensorFieldType::UInt16,
            4 => TensorFieldType::Int16,
            5 => TensorFieldType::UInt32,
            6 => TensorFieldType::Int32,
            7 => TensorFieldType::UInt64,
            8 => TensorFieldType::Int64,
            9 => TensorFieldType::Float16,
            10 => TensorFieldType::Float32,
            11 => TensorFieldType::Float64,
            _ => TensorFieldType::Invalid,
        }
    }
}

impl TensorFieldType {
    pub fn size(&self) -> usize {
        match self {
            TensorFieldType::Invalid => 0,
            TensorFieldType::UInt8 => 1,
            TensorFieldType::Int8 => 1,
            TensorFieldType::UInt16 => 2,
            TensorFieldType::Int16 => 2,
            TensorFieldType::UInt32 => 4,
            TensorFieldType::Int32 => 4,
            TensorFieldType::UInt64 => 8,
            TensorFieldType::Int64 => 8,
            TensorFieldType::Float16 => 2,
            TensorFieldType::Float32 => 4,
            TensorFieldType::Float64 => 8,
        }
    }
}

#[derive(Debug)]
pub struct TensorField {
    dtype: TensorFieldType,
    offset: usize,
    shape: Vec<usize>,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct TensorFile {
    pub fields: HashMap<String, TensorField>,
    pub size: usize,
}

impl TensorFile {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let size = file.metadata()?.len() as usize;
        let mut reader = BufReader::new(file);
        let mut fields = HashMap::default();

        // let raw = fs::read(path)?;
        if size < 12 + 2 + 4 {
            Err("Invalid tensor file: too small :<")?;
        }
        let mut header = [0; 12];
        reader.read_exact(&mut header)?;
        // utf8 is equal to ascii here
        if std::str::from_utf8(&header)? != "tensor_file\0" {
            Err("Invalid tensor file: bad header ;;")?;
        }
        let mut version = [0; 2];
        reader.read_exact(&mut version)?;
        if version[0] != 1 || version[1] != 0 {
            Err("Invalid tensor file: unknown version :(")?;
        }

        let n_fields = {
            let mut bytes = [0; 4];
            reader.read_exact(&mut bytes)?;
            u32::from_le_bytes(bytes)
        };

        for _ in 0..n_fields {
            let name_len = {
                let mut data = [0; 2];
                reader.read_exact(&mut data)?;
                u16::from_le_bytes(data)
            } as usize;
            let name = {
                let mut data = vec![0; name_len];
                reader.read_exact(&mut data)?;
                String::from_utf8(data)?
            };

            let ndim = {
                let mut data = [0; 2];
                reader.read_exact(&mut data)?;
                u16::from_le_bytes(data)
            } as usize;

            let dtype = {
                let mut data = [0; 1];
                reader.read_exact(&mut data)?;
                let dtype = TensorFieldType::from(data[0]);
                if dtype == TensorFieldType::Invalid {
                    Err(format!(
                        "Invalid tensor file: unknown datatype in {name} ;;"
                    ))?
                }
                dtype
            };

            let offset = {
                let mut data = [0; 8];
                reader.read_exact(&mut data)?;
                u64::from_le_bytes(data)
            } as usize;

            let mut shape = Vec::with_capacity(ndim);
            let mut total_size = dtype.size();
            for i in 0..ndim {
                let sizev = {
                    let mut data = [0; 8];
                    reader.read_exact(&mut data)?;
                    u64::from_le_bytes(data)
                } as usize;
                shape.push(sizev);
                total_size *= shape[i];
            }
            let pos = reader.stream_position()?;
            let mut data = vec![0; total_size];
            reader.seek(SeekFrom::Start(offset as u64))?;
            reader.read_exact(&mut data)?;

            reader.seek(SeekFrom::Start(pos))?;
            fields.insert(
                name,
                TensorField {
                    dtype,
                    offset,
                    shape,
                    data,
                },
            );
        }

        Ok(TensorFile { fields, size })
    }
}

#[derive(Debug, Clone)]
pub struct Measured {
    ndf: Warp2D0,
    sigma: Warp2D0,
    vndf: Warp2D2,
    luminance: Warp2D2,
    rgb: Warp2D3,
    isotropic: bool,
    jacobian: bool,
}

#[derive(Debug)]
pub struct SampleResult {
    fr: vec3,
    wo: vec3,
    pdf: f32,
}

fn elevation(d: &vec3) -> f32 {
    // d.z.acos() ==
    2.0 * (0.5 * ((d.x * d.x) + (d.y * d.y) + ((d.z - 1.0) * (d.z - 1.0))).sqrt()).asin()
}

fn u2theta(u: f32) -> f32 {
    u * u * (std::f32::consts::PI / 2.0)
}

fn u2phi(u: f32) -> f32 {
    (2.0 * u - 1.0) * std::f32::consts::PI
}

fn theta2u(theta: f32) -> f32 {
    (theta * (2.0 / std::f32::consts::PI)).sqrt()
}

fn phi2u(phi: f32) -> f32 {
    (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI)
}

const SAMPLE_LUMINANCE: bool = true;

impl Measured {
    pub fn load_from_rgl_rgb_bsdf(path: &Path) -> Result<Measured, Box<dyn Error>> {
        let tf = TensorFile::load_from_file(path)?;
        let theta_i = tf
            .fields
            .get("theta_i")
            .ok_or("bad tensor file, no theta_i")?;
        let phi_i = tf.fields.get("phi_i").ok_or("bad tensor file, no phi_i")?;
        let ndf = tf.fields.get("ndf").ok_or("bad tensor file, no ndf")?;
        let sigma = tf.fields.get("sigma").ok_or("bad tensor file, no sigma")?;
        let vndf = tf.fields.get("vndf").ok_or("bad tensor file, no vndf")?;
        let rgb = tf.fields.get("rgb").ok_or("bad tensor file, no rgb")?;
        let luminance = tf
            .fields
            .get("luminance")
            .ok_or("bad tensor file, no luminance")?;
        let description = tf
            .fields
            .get("description")
            .ok_or("bad tensor file, no description")?;
        let jacobian = tf
            .fields
            .get("jacobian")
            .ok_or("bad tensor file, no jacobian")?;

        // handle errors.
        {
            if !(description.shape.len() == 1 && description.dtype == TensorFieldType::UInt8) {
                Err("Invalid tensor file: bad description.")?;
            }
            if !(theta_i.shape.len() == 1 && theta_i.dtype == TensorFieldType::Float32) {
                Err("Invalid tensor file: bad theta_i.")?;
            }
            if !(phi_i.shape.len() == 1 && phi_i.dtype == TensorFieldType::Float32) {
                Err("Invalid tensor file: bad phi_i.")?;
            }
            if !(ndf.shape.len() == 2 && ndf.dtype == TensorFieldType::Float32) {
                Err("Invalid tensor file: bad ndf.")?;
            }
            if !(sigma.shape.len() == 2 && sigma.dtype == TensorFieldType::Float32) {
                Err("Invalid tensor file: bad sigma.")?;
            }
            if !(vndf.shape.len() == 4
                && vndf.dtype == TensorFieldType::Float32
                && vndf.shape[0] == phi_i.shape[0]
                && vndf.shape[1] == theta_i.shape[0])
            {
                Err("Invalid tensor file: bad vndf.")?;
            }
            if !(luminance.shape.len() == 4
                && luminance.dtype == TensorFieldType::Float32
                && luminance.shape[0] == phi_i.shape[0]
                && luminance.shape[1] == theta_i.shape[0]
                && luminance.shape[2] == luminance.shape[3])
                && luminance.shape[3] == rgb.shape[4]
            {
                Err("Invalid tensor file: bad luminance.")?;
            }
            if !(rgb.dtype == TensorFieldType::Float32
                && rgb.shape.len() == 5
                && rgb.shape[0] == phi_i.shape[0]
                && rgb.shape[1] == theta_i.shape[0]
                && rgb.shape[2] == 3
                && rgb.shape[3] == luminance.shape[2])
            {
                Err("Invalid tensor file: bad rgb.")?;
            }
            if !(jacobian.shape.len() == 1
                && jacobian.shape[0] == 1
                && jacobian.dtype == TensorFieldType::UInt8)
            {
                Err("Invalid tensor file: bad jacobian.")?;
            }
        }

        let isotropic = phi_i.shape[0] <= 2;
        let jacobian = jacobian.data[0] != 0;

        let phi_i_data = phi_i
            .data
            .chunks(4)
            .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
            .collect::<Vec<_>>();

        let theta_i_data = theta_i
            .data
            .chunks(4)
            .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
            .collect::<Vec<_>>();

        if !isotropic
            && (std::f32::consts::TAU / (phi_i_data[phi_i.shape[0] - 1] - phi_i_data[0])) != 1.0
        {
            Err("Reduction != 1.0 not supported")?;
        }

        let ndf = Warp2D0::new(
            &vec2::new(ndf.shape[1] as f32, ndf.shape[0] as f32),
            &ndf.data
                .chunks(4)
                .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
                .collect::<Vec<_>>(),
            [],
            false,
            false,
        )?;

        let sigma = Warp2D0::new(
            &vec2::new(sigma.shape[1] as f32, sigma.shape[0] as f32),
            &sigma
                .data
                .chunks(4)
                .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
                .collect::<Vec<_>>(),
            [],
            false,
            false,
        )?;

        let vndf = Warp2D2::new(
            &vec2::new(vndf.shape[3] as f32, vndf.shape[2] as f32),
            &vndf
                .data
                .chunks(4)
                .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
                .collect::<Vec<_>>(),
            [phi_i_data.clone(), theta_i_data.clone()],
            true,
            true,
        )?;

        let luminance = Warp2D2::new(
            &vec2::new(luminance.shape[3] as f32, luminance.shape[2] as f32),
            &luminance
                .data
                .chunks(4)
                .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
                .collect::<Vec<_>>(),
            [phi_i_data.clone(), theta_i_data.clone()],
            true,
            true,
        )?;

        let channels = vec![0.0, 1.0, 2.0];
        let rgb = Warp2D3::new(
            &vec2::new(rgb.shape[4] as f32, rgb.shape[3] as f32),
            &rgb.data
                .chunks(4)
                .map(|x| f32::from_le_bytes(x.try_into().expect("oop")))
                .collect::<Vec<_>>(),
            [phi_i_data, theta_i_data, channels],
            false,
            false,
        )?;

        Ok(Measured {
            ndf,
            sigma,
            vndf,
            luminance,
            rgb,
            isotropic,
            jacobian,
        })
    }

    pub fn sample(&self, u: &vec2, wi: &vec3) -> Option<SampleResult> {
        if wi.z <= 0.0 {
            None
        } else {
            let theta_i = elevation(wi);
            let phi_i = wi.y.atan2(wi.x);

            let params = [phi_i, theta_i];
            let u_wi = vec2::new(theta2u(theta_i), phi2u(phi_i));
            let (sample, lum_pdf) = {
                if SAMPLE_LUMINANCE {
                    self.luminance.sample(vec2::new(u.y, u.x), &params)
                } else {
                    (vec2::new(u.y, u.x), 1.0)
                }
            };
            let (u_wm, ndf_pdf) = self.vndf.sample(sample, &params);

            let phi_m = u2phi(u_wm.y) + if self.isotropic { phi_i } else { 0.0 };
            let theta_m = u2theta(u_wm.x);

            let sin_phi_m = phi_m.sin();
            let cos_phi_m = phi_m.cos();
            let sin_theta_m = theta_m.sin();
            let cos_theta_m = theta_m.cos();

            let wm = vec3::new(
                cos_phi_m * sin_theta_m,
                sin_phi_m * sin_theta_m,
                cos_theta_m,
            );

            let wo = wm * 2.0 * wm.dot(*wi) - wi;
            if wo.z <= 0.0 {
                return None;
            }

            let fr = vec3::new_with(|i| {
                let params_fr = [phi_i, theta_i, i as f32];
                self.rgb.eval(sample, &params_fr).max(0.0)
            }) * self.ndf.eval(u_wm, &params)
                / (4.0 * self.sigma.eval(u_wi, &params));

            let jacobian =
                (2.0 * std::f32::consts::PI * std::f32::consts::PI * u_wm.x * sin_theta_m)
                    .max(1e-6)
                    * 4.0
                    * wi.dot(wm);
            let pdf = ndf_pdf * lum_pdf / jacobian;

            Some(SampleResult {
                fr: fr / pdf,
                wo,
                pdf,
            })
        }
    }

    fn to_local(v: &vec3, t: &vec3, bt: &vec3, n: &vec3) -> vec3 {
        vec3::new(v.dot(*bt), v.dot(*t), v.dot(*n))
    }

    fn to_world(v: &vec3, t: &vec3, bt: &vec3, n: &vec3) -> vec3 {
        vec3::new(
            bt.x * v.x + t.x * v.y + n.x * v.z,
            bt.y * v.x + t.y * v.y + n.y * v.z,
            bt.z * v.x + t.z * v.y + n.z * v.z,
        )
    }
}

impl Scatterable for Measured {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let n = hit.normal;
        let t = n
            // .cross(vec3::random_in_hemisphere(&n).normalized())
            .cross(ray.direction.normalized())
            .normalized();
        let bt = t.cross(n).normalized();
        let wi_local = Self::to_local(&-ray.direction.normalized(), &t, &bt, &n);

        let s = self.sample(&vec2::random(), &wi_local.normalized())?;

        let wo_world = Self::to_world(&s.wo, &t, &bt, &n).normalized();
        // let wo_world = s.wo.normalized();

        Some(Scatter {
            attenuation: (s.fr).into(),
            pdf: None, //Some(CosinePDF::new(wo_world).into()),
            specular: Some(Ray::new(hit.p, wo_world)),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let brdf = Measured::load_from_rgl_rgb_bsdf(Path::new(
            "C:/dev/heliochrome/assets/bsdfs/chm_mint_rgb.bsdf",
        ))?;
        {
            let u = vec2::new(0.0, 0.0);
            let wi = vec3::new(0.381, 0.889001, 0.254).normalized();

            let res = brdf.sample(&u, &wi).ok_or("oof")?;

            dbg!(res.fr);
            dbg!(res.wo);
            dbg!(res.pdf);
        }

        Ok(())
    }

    #[test]
    fn scale() -> Result<(), Box<dyn Error>> {
        let brdf =
            Measured::load_from_rgl_rgb_bsdf(Path::new("../assets/bsdfs/chm_mint_rgb.bsdf"))?;
        let size = 10;
        println!("{{ \"data\": [");
        for ux in 0..size {
            for uy in 0..size {
                for wix in -size..size {
                    for wiy in -size..size {
                        for wiz in -size..size {
                            let u = vec2::new(ux as f32 / size as f32, uy as f32 / size as f32);
                            let wi = &(vec3::new(
                                wix as f32 / size as f32,
                                wiy as f32 / size as f32,
                                wiz as f32 / size as f32,
                            ))
                            .normalized();

                            let res = brdf.sample(&u, wi).unwrap_or(SampleResult {
                                fr: vec3::splat(0.0),
                                wo: vec3::splat(0.0),
                                pdf: 0.0,
                            });
                            println!("{{");
                            {
                                println!("\t\"u.x\": {},", u.x);
                                println!("\t\"u.y\": {},", u.y);

                                println!("\t\"wi.x\": {},", wi.x);
                                println!("\t\"wi.y\": {},", wi.y);
                                println!("\t\"wi.z\": {},", wi.z);

                                println!("\t\"res.fr[0]\": {},", res.fr[0]);
                                println!("\t\"res.fr[1]\": {},", res.fr[1]);
                                println!("\t\"res.fr[2]\": {},", res.fr[2]);
                                println!("\t\"res.wo[0]\": {},", res.wo[0]);
                                println!("\t\"res.wo[1]\": {},", res.wo[1]);
                                println!("\t\"res.wo[2]\": {},", res.wo[2]);
                                println!("\t\"res.pdf\": {}", res.pdf);
                            }
                            println!("}},");
                        }
                    }
                }
            }
        }
        println!("] }}");
        Ok(())
    }
}
