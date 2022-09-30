use crate::{
    color::Color,
    image::Image,
    maths::{mat3, vec3},
};

#[derive(PartialEq, Clone, Copy)]
pub enum ToneMap {
    Clamp,
    Simple(f32),
    Hable,
    Reinhard(f32),
    HejlRichard,
    ACES,
}

impl ToneMap {
    pub fn clamp(img: &mut Image) {
        img.buffer.iter_mut().for_each(|c| *c = c.clamp(0.0, 1.0));
    }

    pub fn simple(img: &mut Image, exposure: f32) {
        img.buffer
            .iter_mut()
            .for_each(|c| *c = Color::splat(1.0) - (-*c * exposure).exp());
    }

    pub fn hable(img: &mut Image) {
        img.buffer.iter_mut().for_each(|c| {
            let f = |x| {
                let a = Color::splat(0.15);
                let b = Color::splat(0.50);
                let c = Color::splat(0.10);
                let d = Color::splat(0.20);
                let e = Color::splat(0.02);
                let f = Color::splat(0.30);
                ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f)) - e / f
            };
            let exposure_bias = 2.0;
            let curr = f(*c * exposure_bias);
            let w = Color::splat(11.2);
            let white_scale = Color::splat(1.0) / f(w);
            *c = curr * white_scale;
        });
    }

    pub fn aces(img: &mut Image) {
        let input = mat3::new([
            vec3::new(0.59719, 0.35458, 0.04823),
            vec3::new(0.07600, 0.90834, 0.01566),
            vec3::new(0.02840, 0.13383, 0.83777),
        ])
        .transposed();

        let output = mat3::new([
            vec3::new(1.60475, -0.53108, -0.07367),
            vec3::new(-0.10208, 1.10813, -0.00605),
            vec3::new(-0.00327, -0.07276, 1.07602),
        ])
        .transposed();

        let rtt_and_odt_fit = |c| {
            let a = c * (c + vec3::splat(0.0245786)) - vec3::splat(0.000090537);
            let b = c * (vec3::splat(0.983729) * c + vec3::splat(0.432951)) + vec3::splat(0.238081);
            a / b
        };

        img.buffer.iter_mut().for_each(|c| {
            let mut v: vec3 = (*c).into();
            v = input * v;
            v = rtt_and_odt_fit(v);
            *c = (output * v).clamp(0.0, 1.0).into()
        });
    }

    pub fn reinhard(img: &mut Image, white_point: f32) {
        img.buffer.iter_mut().for_each(|c| {
            let l_old = c.luminance();
            let n = l_old * (1.0 + (l_old / (white_point * white_point)));
            let l_new = n / (1.0 + l_old);

            *c = c.change_luminance(l_new);

            // let n = *c * (Color::splat(1.0) + (*c / Color::splat(white_point * white_point)));
            // *c = n / (Color::splat(1.0) + *c);
        });
    }

    pub fn hejl_richard(img: &mut Image) {
        img.buffer.iter_mut().for_each(|c| {
            *c = Color::splat(0.0).max(&(*c - Color::splat(0.004)));
            *c = (*c * (6.2 * *c + Color::splat(0.5)))
                / (*c * (6.2 * *c + Color::splat(1.7)) + Color::splat(0.06));
        });
    }

    pub fn map(&self, img: &mut Image) {
        match self {
            ToneMap::Clamp => ToneMap::clamp(img),
            ToneMap::Simple(exposure) => ToneMap::simple(img, *exposure),
            ToneMap::Reinhard(white_point) => ToneMap::reinhard(img, *white_point),
            ToneMap::HejlRichard => ToneMap::hejl_richard(img),
            ToneMap::Hable => ToneMap::hable(img),
            ToneMap::ACES => ToneMap::aces(img),
        }
    }
}

impl ToString for ToneMap {
    fn to_string(&self) -> String {
        match self {
            ToneMap::Clamp => "Clamp",
            ToneMap::Simple(_) => "Simple",
            ToneMap::Reinhard(_) => "Reinhard",
            ToneMap::HejlRichard => "HejlRichard",
            ToneMap::Hable => "Hable",
            ToneMap::ACES => "ACES",
        }
        .to_owned()
    }
}
