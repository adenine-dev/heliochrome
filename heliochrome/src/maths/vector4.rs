use std::ops;
use std::ops::*;

use impl_ops::*;
use rand::distributions::{Distribution, Uniform};

use crate::maths::{misc::*, vec3};

macro_rules! vec4_impl {
    ($n:ident, $t:ident, $x:ident, $y:ident, $z:ident, $w:ident, $v3:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $n {
            pub $x: $t,
            pub $y: $t,
            pub $z: $t,
            pub $w: $t,
        }

        impl $n {
            pub const fn new($x: $t, $y: $t, $z: $t, $w: $t) -> Self {
                Self { $x, $y, $z, $w }
            }

            pub const fn splat(s: $t) -> Self {
                Self {
                    $x: s,
                    $y: s,
                    $z: s,
                    $w: s,
                }
            }

            pub const fn from_vec3(v3: $v3, $w: $t) -> Self {
                Self {
                    $x: v3.$x,
                    $y: v3.$y,
                    $z: v3.$z,
                    $w
                }
            }

            pub const fn unit_x() -> Self {
                Self {
                    $x: $t::splat(1.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(0.0),
                    $w: $t::splat(0.0),
                }
            }

            pub const fn unit_y() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(1.0),
                    $z: $t::splat(0.0),
                    $w: $t::splat(0.0),
                }
            }

            pub const fn unit_z() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(1.0),
                    $w: $t::splat(0.0),
                }
            }

            pub const fn unit_w() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(0.0),
                    $z: $t::splat(0.0),
                    $w: $t::splat(1.0),
                }
            }

            pub const fn to_vec3(self) -> $v3 {
                $v3::new(self.$x, self.$y, self.$z)
            }

            pub fn dot(&self, other: $n) -> $t {
                (self.$x * other.$x) + (self.$y * other.$y) + (self.$z * other.$z) + (self.$w * other.$w)
            }

            pub fn random() -> Self {
                Self::new(
                    rand::random::<$t>(),
                    rand::random::<$t>(),
                    rand::random::<$t>(),
                    rand::random::<$t>(),
                )
            }

            pub fn random_in_unit_sphere() -> Self {
                let mut rng = rand::thread_rng();
                let s = Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let ret = Self::new(s.sample(&mut rng), s.sample(&mut rng), s.sample(&mut rng), s.sample(&mut rng));
                    if ret.mag_sq() < 1.0 {
                        return ret;
                    }
                }
            }

            pub fn near_zero(&self) -> bool {
                self.$x.abs() < $t::EPSILON
                    && self.$y.abs() < $t::EPSILON
                    && self.$z.abs() < $t::EPSILON
                    && self.$w.abs() < $t::EPSILON
            }

            pub fn mag_sq(&self) -> $t {
                (self.$x * self.$x) + (self.$y * self.$y) + (self.$z * self.$z) + (self.$w * self.$w)
            }

            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            pub fn normalize(&mut self) -> Self {
                *self /= self.mag();
                *self
            }

            pub fn sqrt(&self) -> Self {
                Self::new(self.$x.sqrt(), self.$y.sqrt(), self.$z.sqrt(), self.$w.sqrt())
            }

            pub fn floor(&self) -> Self {
                Self::new(self.$x.floor(), self.$y.floor(), self.$z.floor(), self.$w.floor())
            }


            pub fn powf(&self, n: $t) -> Self {
                Self::new(self.$x.powf(n), self.$y.powf(n), self.$z.powf(n), self.$w.powf(n))
            }

            pub fn normalized(&self) -> Self {
                self.clone().normalize()
            }

            pub fn min(&self, other: &$n) -> $n {
                Self::new(self.$x.min(other.$x), self.$y.min(other.$y), self.$z.min(other.$z), self.$w.min(other.$w))
            }

            pub fn max(&self, other: &$n) -> $n {
                Self::new(self.$x.max(other.$x), self.$y.max(other.$y), self.$z.max(other.$z), self.$w.max(other.$w))
            }

            pub fn clamp(&self, min: $t, max: $t) -> $n {
                Self::new(self.$x.clamp(min, max), self.$y.clamp(min, max), self.$z.clamp(min, max), self.$w.clamp(min, max))
            }

            pub fn exp(&self) -> $n {
                Self::new(self.$x.exp(), self.$y.exp(), self.$z.exp(), self.$w.exp())
            }
        }

        impl Index<usize> for $n {
            type Output = $t;
            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    0 => &self.$x,
                    1 => &self.$y,
                    2 => &self.$z,
                    3 => &self.$w,
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
                    3 => &mut self.$w,
                    _ => panic!("index out of bounds")
                }
            }
        }

        impl_op_ex!(+ |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x + rhs.$x, lhs.$y + rhs.$y, lhs.$z + rhs.$z, lhs.$w + rhs.$w)
        });

        impl_op_ex!(- |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x - rhs.$x, lhs.$y - rhs.$y, lhs.$z - rhs.$z, lhs.$w - rhs.$w)
        });

        impl_op_ex!(* |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x * rhs.$x, lhs.$y * rhs.$y, lhs.$z * rhs.$z, lhs.$w * rhs.$w)
        });

        impl_op_ex!(/ |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x / rhs.$x, lhs.$y / rhs.$y, lhs.$z / rhs.$z, lhs.$w / rhs.$w)
        });

        impl_op_ex!(% |lhs: &$n, rhs: &$n| -> $n {
            $n::new(lhs.$x % rhs.$x, lhs.$y % rhs.$y, lhs.$z % rhs.$z, lhs.$w % rhs.$w)
        });

        impl_op_ex_commutative!(* |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x * rhs, lhs.$y * rhs, lhs.$z * rhs, lhs.$w * rhs)
        });

        impl_op_ex_commutative!(/ |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x / rhs, lhs.$y / rhs, lhs.$z / rhs, lhs.$w / rhs)
        });

        impl_op_ex_commutative!(% |lhs: &$n, rhs: &$t| -> $n {
            $n::new(lhs.$x % rhs, lhs.$y % rhs, lhs.$z % rhs, lhs.$w % rhs)
        });


        impl_op_ex!(+= |lhs: &mut $n, rhs: &$n| {
            lhs.$x += rhs.$x;
            lhs.$y += rhs.$y;
            lhs.$z += rhs.$z;
            lhs.$w += rhs.$w;
        });

        impl_op_ex!(-= |lhs: &mut $n, rhs: &$n| {
            lhs.$x -= rhs.$x;
            lhs.$y -= rhs.$y;
            lhs.$z -= rhs.$z;
            lhs.$w -= rhs.$w;
        });

        impl_op_ex!(*= |lhs: &mut $n, rhs: &$n| {
            lhs.$x *= rhs.$x;
            lhs.$y *= rhs.$y;
            lhs.$z *= rhs.$z;
            lhs.$w *= rhs.$w;
        });

        impl_op_ex!(/= |lhs: &mut $n, rhs: &$n| {
            lhs.$x /= rhs.$x;
            lhs.$y /= rhs.$y;
            lhs.$z /= rhs.$z;
            lhs.$w /= rhs.$w;
        });

        impl_op_ex!(*= |lhs: &mut $n, rhs: &$t| {
            lhs.$x *= rhs;
            lhs.$y *= rhs;
            lhs.$z *= rhs;
            lhs.$w *= rhs;
        });

        impl_op_ex!(/= |lhs: &mut $n, rhs: &$t| {
            lhs.$x /= rhs;
            lhs.$y /= rhs;
            lhs.$z /= rhs;
            lhs.$w /= rhs;
        });

        impl_op_ex!(- |lhs: &$n| -> $n {
            $n::new(-lhs.$x, -lhs.$y, -lhs.$z, -lhs.$w)
        });
    };
}

pub(crate) use vec4_impl;

vec4_impl!(vec4, f32, x, y, z, w, vec3);
