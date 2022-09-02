use crate::maths::misc::*;
use core::ops::*;

macro_rules! vec3_impl {
    ($n:ident, $t:ident, $x:ident, $y:ident, $z:ident) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
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

            pub fn normalized(&self) -> Self {
                self.clone().normalize()
            }
        }

        impl Add for $n {
            type Output = Self;
            fn add(self, rhs: $n) -> Self {
                $n::new(self.$x + rhs.$x, self.$y + rhs.$y, self.$z + rhs.$z)
            }
        }

        impl AddAssign for $n {
            fn add_assign(&mut self, rhs: $n) {
                self.$x += rhs.$x;
                self.$y += rhs.$y;
                self.$z += rhs.$z;
            }
        }

        impl Sub for $n {
            type Output = Self;
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.$x - rhs.$x, self.$y - rhs.$y, self.$z - rhs.$z)
            }
        }

        impl SubAssign for $n {
            fn sub_assign(&mut self, rhs: $n) {
                self.$x -= rhs.$x;
                self.$y -= rhs.$y;
                self.$z -= rhs.$z;
            }
        }

        impl Mul for $n {
            type Output = Self;
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.$x * rhs.$x, self.$y * rhs.$y, self.$z * rhs.$z)
            }
        }

        impl MulAssign for $n {
            fn mul_assign(&mut self, rhs: $n) {
                self.$x *= rhs.$x;
                self.$y *= rhs.$y;
                self.$z *= rhs.$z;
            }
        }

        impl Mul<$t> for $n {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self {
                $n::new(self.$x * rhs, self.$y * rhs, self.$z * rhs)
            }
        }

        impl MulAssign<$t> for $n {
            fn mul_assign(&mut self, rhs: $t) {
                self.$x *= rhs;
                self.$y *= rhs;
                self.$z *= rhs;
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.$x, self * rhs.$y, self * rhs.$z)
            }
        }

        impl Div for $n {
            type Output = Self;
            fn div(self, rhs: $n) -> Self {
                $n::new(self.$x * rhs.$x, self.$y * rhs.$y, self.$z * rhs.$z)
            }
        }

        impl DivAssign for $n {
            fn div_assign(&mut self, rhs: $n) {
                self.$x /= rhs.$x;
                self.$y /= rhs.$y;
                self.$z /= rhs.$z;
            }
        }

        impl Div<$t> for $n {
            type Output = Self;
            fn div(self, rhs: $t) -> Self {
                $n::new(self.$x / rhs, self.$y / rhs, self.$z / rhs)
            }
        }

        impl DivAssign<$t> for $n {
            fn div_assign(&mut self, rhs: $t) {
                self.$x /= rhs;
                self.$y /= rhs;
                self.$z /= rhs;
            }
        }

        impl Div<$n> for $t {
            type Output = $n;
            fn div(self, rhs: $n) -> $n {
                $n::new(self / rhs.$x, self / rhs.$y, self / rhs.$z)
            }
        }

        impl Neg for $n {
            type Output = $n;
            fn neg(self) -> $n {
                $n::new(-self.$x, -self.$y, -self.$z)
            }
        }
    };
}

pub(crate) use vec3_impl;

vec3_impl!(vec3, f32, x, y, z);
vec3_impl!(dvec3, f64, x, y, z);
