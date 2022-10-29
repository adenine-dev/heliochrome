use std::ops;
use std::ops::*;

use impl_ops::*;
use rand::distributions::{Distribution, Uniform};

use crate::maths::misc::*;

macro_rules! vec3_impl {
    ($n:ident, $t:ident, $x:ident, $y:ident, $z:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $n {
            pub $x: $t,
            pub $y: $t,
            pub $z: $t,
        }

        impl $n {
            pub const fn new($x: $t, $y: $t, $z: $t) -> Self {
                Self { $x, $y, $z }
            }

            pub const fn splat(s: $t) -> Self {
                Self {
                    $x: s,
                    $y: s,
                    $z: s,
                }
            }

            pub const fn unit_x() -> Self {
                Self {
                    $x: $t::splat(1.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(0.0),
                }
            }

            pub const fn unit_y() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(1.0),
                    $z: $t::splat(0.0),
                }
            }

            pub const fn unit_z() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(1.0),
                }
            }

            pub fn dot(&self, other: $n) -> $t {
                (self.$x * other.$x) + (self.$y * other.$y) + (self.$z * other.$z)
            }

            pub fn cross(&self, other: $n) -> Self {
                $n::new(
                    (self.$y * other.$z) + (-self.$z * other.$y),
                    (self.$z * other.$x) + (-self.$x * other.$z),
                    (self.$x * other.$y) + (-self.$y * other.$x),
                )
            }

            pub fn random() -> Self {
                Self::new(
                    rand::random::<$t>(),
                    rand::random::<$t>(),
                    rand::random::<$t>(),
                )
            }

            pub fn random_in_unit_xy_disk() -> Self {
                let mut rng = rand::thread_rng();
                let s = Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let ret = Self::new(s.sample(&mut rng), s.sample(&mut rng), 0.0);
                    if ret.mag_sq() < 1.0 {
                        return ret;
                    }
                }

            }

            pub fn random_in_unit_sphere() -> Self {
                let mut rng = rand::thread_rng();
                let s = Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let ret = Self::new(s.sample(&mut rng), s.sample(&mut rng), s.sample(&mut rng));
                    if ret.mag_sq() < 1.0 {
                        return ret;
                    }
                }
            }

            pub fn random_in_hemisphere(normal: &Self) -> Self {
                let v = Self::random_in_unit_sphere();
                if v.dot(*normal) > 0.0 {
                    v
                } else {
                    -v
                }
            }

            pub fn random_cosine_direction() -> Self {
                let r1 = rand::random::<$t>();
                let r2 = rand::random::<$t>();
                let z = (1.0 - r2).sqrt();

                let phi = std::$t::consts::TAU * r1;
                let x = phi.cos() * r2.sqrt();
                let y = phi.sin() * r2.sqrt();

                Self::new(x, y, z)
            }

            pub fn random_to_sphere(radius: $t, dist_sq: $t) -> Self {
                let r1 = rand::random::<$t>();
                let r2 = rand::random::<$t>();
                let z = 1.0 + r2*((1.0-radius*radius/dist_sq).sqrt() - 1.0);

                let phi = std::$t::consts::TAU*r1;
                let x = phi.cos()*(1.0-z*z).sqrt();
                let y = phi.sin()*(1.0-z*z).sqrt();

                Self::new(x, y, z)
            }

            pub fn new_with<F: FnMut(usize) -> $t>(mut f: F) -> Self {
                Self::new(f(0), f(1), f(2))
            }

            pub fn near_zero(&self) -> bool {
                self.$x.abs() < $t::EPSILON
                    && self.$y.abs() < $t::EPSILON
                    && self.$z.abs() < $t::EPSILON
            }

            pub fn mag_sq(&self) -> $t {
                (self.$x * self.$x) + (self.$y * self.$y) + (self.$z * self.$z)
            }

            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            pub fn normalize(&mut self) -> Self {
                *self /= self.mag();
                *self
            }

            pub fn sqrt(&self) -> Self {
                Self::new(self.$x.sqrt(), self.$y.sqrt(), self.$z.sqrt())
            }

            pub fn abs(&self) -> Self {
                Self::new(self.$x.abs(), self.$y.abs(), self.$z.abs())
            }

            pub fn floor(&self) -> Self {
                Self::new(self.$x.floor(), self.$y.floor(), self.$z.floor())
            }

            pub fn powf(&self, n: $t) -> Self {
                Self::new(self.$x.powf(n), self.$y.powf(n), self.$z.powf(n))
            }

            pub fn normalized(&self) -> Self {
                self.clone().normalize()
            }

            pub fn reflect_over(&self, n: $n) -> Self {
                self - 2.0 * self.dot(n) * n
            }

            pub fn refract(&self, n: $n, etai_over_etat: $t) -> $n {
                let cos_theta = (-self).dot(n).min(1.0);
                let r_out_perp = etai_over_etat * (self + cos_theta * n);
                let r_out_parallel = -(1.0 - r_out_perp.mag_sq()).abs().sqrt() * n;
                r_out_perp + r_out_parallel
            }

            pub fn project_on(&self, n: $n) -> $n {
                (self.dot(n) / n.dot(n)) * n
            }

            pub fn un_nan(mut self) -> $n {
                if self.$x != self.$x {self.$x = 0.0;}
                if self.$y != self.$y {self.$y = 0.0;}
                if self.$z != self.$z {self.$z = 0.0;}
                self
            }

            pub fn min(&self, other: &$n) -> $n {
                Self::new(self.$x.min(other.$x), self.$y.min(other.$y), self.$z.min(other.$z))
            }

            pub fn max(&self, other: &$n) -> $n {
                Self::new(self.$x.max(other.$x), self.$y.max(other.$y), self.$z.max(other.$z))
            }

            pub fn clamp(&self, min: $t, max: $t) -> $n {
                Self::new(self.$x.clamp(min, max), self.$y.clamp(min, max), self.$z.clamp(min, max))
            }

            pub fn signum(&self) -> $n {
                Self::new(self.$x.signum(), self.$y.signum(), self.$z.signum())
            }

            pub fn exp(&self) -> $n {
                Self::new(self.$x.exp(), self.$y.exp(), self.$z.exp())
            }
        }

        impl Index<usize> for $n {
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

vec3_impl!(vec3, f32, x, y, z);
vec3_impl!(dvec3, f64, x, y, z);
