use std::{
    ops,
    ops::*,
    simd::{f32x4, SimdFloat, StdFloat},
    str::FromStr,
};

use impl_ops::*;
use rand::distributions::{Distribution, Uniform};

use crate::maths::misc::*;

macro_rules! vec3_impl {
    ($n:ident, $t:ident, $b:ident, $x:ident, $y:ident, $z:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $n {
            pub $x: $t,
            pub $y: $t,
            pub $z: $t,
        }

        impl $n {
            #[inline]
            pub const fn new($x: $t, $y: $t, $z: $t) -> Self {
                Self { $x, $y, $z }
            }

            #[inline]
            pub const fn splat(s: $t) -> Self {
                Self {
                    $x: s,
                    $y: s,
                    $z: s,
                }
            }

            #[inline]
            pub fn unit_x() -> Self {
                Self {
                    $x: $t::splat(1.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(0.0),
                }
            }

            #[inline]
            pub fn unit_y() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(1.0),
                    $z: $t::splat(0.0),
                }
            }

            #[inline]
            pub fn unit_z() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(1.0),
                }
            }

            #[inline]
            pub fn dot(&self, other: $n) -> $t {
                (self.$x * other.$x) + (self.$y * other.$y) + (self.$z * other.$z)
            }

            #[inline]
            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    (self.$y * other.$z) + (-self.$z * other.$y),
                    (self.$z * other.$x) + (-self.$x * other.$z),
                    (self.$x * other.$y) + (-self.$y * other.$x),
                )
            }

            #[inline]
            pub fn random() -> Self {
                Self::new(
                    $t::splat(rand::random::<$b>()),
                    $t::splat(rand::random::<$b>()),
                    $t::splat(rand::random::<$b>()),
                )
            }

            #[inline]
            pub fn random_in_unit_xy_disk() -> Self {
                let mut rng = rand::thread_rng();
                let s = Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let ret = Self::new($t::splat(s.sample(&mut rng)), $t::splat(s.sample(&mut rng)), $t::splat(0.0));
                    if ret.mag_sq() < $t::splat(1.0) {
                        return ret;
                    }
                }

            }

            #[inline]
            pub fn random_in_unit_sphere() -> Self {
                let mut rng = rand::thread_rng();
                let s = Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let ret = Self::new($t::splat(s.sample(&mut rng)), $t::splat(s.sample(&mut rng)), $t::splat(s.sample(&mut rng)));
                    if ret.mag_sq() < $t::splat(1.0) {
                        return ret;
                    }
                }
            }

            #[inline]
            pub fn random_in_hemisphere(normal: &Self) -> Self {
                let v = Self::random_in_unit_sphere();
                if v.dot(*normal) > $t::splat(0.0) {
                    v
                } else {
                    -v
                }
            }

            #[inline]
            pub fn near_zero(&self) -> bool {
                self.$x.abs() < $t::splat($b::EPSILON)
                    && self.$y.abs() < $t::splat($b::EPSILON)
                    && self.$z.abs() < $t::splat($b::EPSILON)
            }

            #[inline]
            pub fn mag_sq(&self) -> $t {
                (self.$x * self.$x) + (self.$y * self.$y) + (self.$z * self.$z)
            }

            #[inline]
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            #[inline]
            pub fn normalize(&mut self) -> Self {
                *self /= self.mag();
                *self
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                self.clone().normalize()
            }

            #[inline]
            pub fn sqrt(&self) -> Self {
                Self::new(self.$x.sqrt(), self.$y.sqrt(), self.$z.sqrt())
            }

            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.$x.abs(), self.$y.abs(), self.$z.abs())
            }


            #[inline]
            pub fn floor(&self) -> Self {
                Self::new(self.$x.floor(), self.$y.floor(), self.$z.floor())
            }

            #[inline]
            pub fn reflect_over(&self, n: $n) -> Self {
                self - Self::splat($t::splat(2.0)) * self.dot(n) * n
            }

            #[inline]
            pub fn refract(&self, n: $n, etai_over_etat: $t) -> $n {
                let cos_theta = (-self).dot(n).min($t::splat(1.0));
                let r_out_perp = etai_over_etat * (self + cos_theta * n);
                let r_out_parallel = -($t::splat(1.0) - r_out_perp.mag_sq()).abs().sqrt() * n;
                r_out_perp + r_out_parallel
            }

            #[inline]
            pub fn un_nan(mut self) -> $n {
                if self.$x != self.$x {self.$x = $t::splat(0.0);}
                if self.$y != self.$y {self.$y = $t::splat(0.0);}
                if self.$z != self.$z {self.$z = $t::splat(0.0);}
                self
            }

            #[inline]
            pub fn project_on(&self, n: $n) -> $n {
                (self.dot(n) / n.dot(n)) * n
            }

            #[inline]
            pub fn min(&self, other: &$n) -> $n {
                Self::new(self.$x.min(other.$x), self.$y.min(other.$y), self.$z.min(other.$z))
            }

            #[inline]
            pub fn max(&self, other: &$n) -> $n {
                Self::new(self.$x.max(other.$x), self.$y.max(other.$y), self.$z.max(other.$z))
            }

            #[inline]
            pub fn clamp(&self, min: $t, max: $t) -> $n {
                Self::new(self.$x.clamp(min, max), self.$y.clamp(min, max), self.$z.clamp(min, max))
            }

            #[inline]
            pub fn signum(&self) -> $n {
                Self::new(self.$x.signum(), self.$y.signum(), self.$z.signum())
            }

            #[inline]
            pub fn recip(&self) -> Self {
                Self::new(self.$x.recip(), self.$y.recip(), self.$z.recip())
            }

            #[inline]
            pub fn min_component(&self) -> $t {
                self.$x.min(self.$y.min(self.$z))
            }

            #[inline]
            pub fn max_component(&self) -> $t {
                self.$x.max(self.$y.max(self.$z))
            }

            #[inline]
            pub fn to_rad(&self) -> $n {
                Self::new(self.$x.to_radians(), self.$y.to_radians(), self.$z.to_radians())
            }

            #[inline]
            pub fn to_deg(&self) -> $n {
                Self::new(self.$x.to_degrees(), self.$y.to_degrees(), self.$z.to_degrees())
            }
        }

        impl const Index<usize> for $n {
            type Output = $t;
            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    0 => &self.$x,
                    1 => &self.$y,
                    2 => &self.$z,
                    _ => panic!("index out of bounds")
                }
            }
        }

        impl IndexMut<usize> for $n {
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                match i {
                    0 => &mut self.$x,
                    1 => &mut self.$y,
                    2 => &mut self.$z,
                    _ => panic!("index out of bounds")
                }
            }
        }

        impl_op_ex!(+ |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x + rhs.$x, lhs.$y + rhs.$y, lhs.$z + rhs.$z)
        });

        impl_op_ex!(- |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x - rhs.$x, lhs.$y - rhs.$y, lhs.$z - rhs.$z)
        });

        impl_op_ex!(* |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x * rhs.$x, lhs.$y * rhs.$y, lhs.$z * rhs.$z)
        });

        impl_op_ex!(/ |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x / rhs.$x, lhs.$y / rhs.$y, lhs.$z / rhs.$z)
        });

        impl_op_ex!(% |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x % rhs.$x, lhs.$y % rhs.$y, lhs.$z % rhs.$z)
        });

        impl_op_ex_commutative!(* |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x * rhs, lhs.$y * rhs, lhs.$z * rhs)
        });

        impl_op_ex_commutative!(/ |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x / rhs, lhs.$y / rhs, lhs.$z / rhs)
        });

        impl_op_ex_commutative!(% |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x % rhs, lhs.$y % rhs, lhs.$z % rhs)
        });


        impl_op_ex!(+= |lhs: &mut $n, rhs: &$n| {
            lhs.$x += rhs.$x;
            lhs.$y += rhs.$y;
            lhs.$z += rhs.$z;
        });

        impl_op_ex!(-= |lhs: &mut $n, rhs: &$n| {
            lhs.$x -= rhs.$x;
            lhs.$y -= rhs.$y;
            lhs.$z -= rhs.$z;
        });

        impl_op_ex!(*= |lhs: &mut $n, rhs: &$n| {
            lhs.$x *= rhs.$x;
            lhs.$y *= rhs.$y;
            lhs.$z *= rhs.$z;
        });

        impl_op_ex!(/= |lhs: &mut $n, rhs: &$n| {
            lhs.$x /= rhs.$x;
            lhs.$y /= rhs.$y;
            lhs.$z /= rhs.$z;
        });

        impl_op_ex!(*= |lhs: &mut $n, rhs: &$t| {
            lhs.$x *= rhs;
            lhs.$y *= rhs;
            lhs.$z *= rhs;
        });

        impl_op_ex!(/= |lhs: &mut $n, rhs: &$t| {
            lhs.$x /= rhs;
            lhs.$y /= rhs;
            lhs.$z /= rhs;
        });

        impl_op_ex!(- |lhs: &$n| -> $n {
            $n::new(-lhs.$x, -lhs.$y, -lhs.$z)
        });
    };
}

pub(crate) use vec3_impl;

vec3_impl!(vec3, f32, f32, x, y, z);
vec3_impl!(mvec3, f32x4, f32, x, y, z);

impl vec3 {
    pub fn random_cosine_direction() -> Self {
        let r1 = rand::random::<f32>();
        let r2 = rand::random::<f32>();
        let z = (1.0 - r2).sqrt();

        let phi = std::f32::consts::TAU * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Self::new(x, y, z)
    }
}

impl FromStr for vec3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split(',').into_iter().collect::<Vec<_>>();
        if iter.len() == 3 {
            Ok(vec3::new(
                iter[0].trim().parse().map_err(|x| format!("{x}"))?,
                iter[1].trim().parse().map_err(|x| format!("{x}"))?,
                iter[2].trim().parse().map_err(|x| format!("{x}"))?,
            ))
        } else {
            Err("invalid vec3 string, unexpected number of components".to_owned())
        }
    }
}
