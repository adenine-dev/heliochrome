use core::ops::*;

use rand::prelude::*;

use crate::maths::misc::*;

macro_rules! vec2_impl {
    ($n:ident, $t:ident, $x:ident, $y:ident) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $n {
            pub $x: $t,
            pub $y: $t,
        }

        impl $n {
            #[inline]
            pub const fn new($x: $t, $y: $t) -> Self {
                Self { $x, $y }
            }

            #[inline]
            pub const fn splat(s: $t) -> Self {
                Self { $x: s, $y: s }
            }

            #[inline]
            pub const fn unit_x() -> Self {
                Self {
                    $x: $t::splat(1.0),
                    $y: $t::splat(0.0),
                }
            }

            #[inline]
            pub const fn unit_y() -> Self {
                Self {
                    $x: $t::splat(0.0),
                    $y: $t::splat(1.0),
                }
            }

            pub fn random() -> Self {
                let mut rng = rand::thread_rng();
                Self::new(rng.gen(), rng.gen()).normalize()
            }

            pub fn dot(&self, other: $n) -> $t {
                (self.$x * other.$x) + (self.$y * other.$y)
            }

            pub fn mag_sq(&self) -> $t {
                (self.$x * self.$x) + (self.$y * self.$y)
            }
            pub fn mag(&self) -> $t {
                self.mag_sq().sqrt()
            }

            pub fn normalize(&mut self) -> Self {
                let mag = $t::splat(1.0) / self.mag();
                *self *= mag;
                *self
            }

            pub fn normalized(&self) -> Self {
                self.clone().normalize()
            }

            pub fn prod(&self) -> $t {
                self.$x * self.$y
            }

            pub fn min(&self, other: &$n) -> $n {
                Self::new(self.$x.min(other.$x), self.$y.min(other.$y))
            }

            pub fn max(&self, other: &$n) -> $n {
                Self::new(self.$x.max(other.$x), self.$y.max(other.$y))
            }

            pub fn clamp(&self, min: $t, max: $t) -> $n {
                Self::new(self.$x.clamp(min, max), self.$y.clamp(min, max))
            }

            pub fn floor(&self) -> Self {
                Self::new(self.$x.floor(), self.$y.floor())
            }

            pub fn abs(&self) -> Self {
                Self::new(self.$x.abs(), self.$y.abs())
            }
        }

        impl Add for $n {
            type Output = Self;
            fn add(self, rhs: $n) -> Self {
                $n::new(self.$x + rhs.$x, self.$y + rhs.$y)
            }
        }

        impl AddAssign for $n {
            fn add_assign(&mut self, rhs: $n) {
                self.$x += rhs.$x;
                self.$y += rhs.$y;
            }
        }

        impl Sub for $n {
            type Output = Self;
            fn sub(self, rhs: $n) -> Self {
                $n::new(self.$x - rhs.$x, self.$y - rhs.$y)
            }
        }

        impl SubAssign for $n {
            fn sub_assign(&mut self, rhs: $n) {
                self.$x -= rhs.$x;
                self.$y -= rhs.$y;
            }
        }

        impl Mul for $n {
            type Output = Self;
            fn mul(self, rhs: $n) -> Self {
                $n::new(self.$x * rhs.$x, self.$y * rhs.$y)
            }
        }

        impl MulAssign for $n {
            fn mul_assign(&mut self, rhs: $n) {
                self.$x *= rhs.$x;
                self.$y *= rhs.$y;
            }
        }

        impl Mul<$t> for $n {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self {
                $n::new(self.$x * rhs, self.$y * rhs)
            }
        }

        impl MulAssign<$t> for $n {
            fn mul_assign(&mut self, rhs: $t) {
                self.$x *= rhs;
                self.$y *= rhs;
            }
        }

        impl Mul<$n> for $t {
            type Output = $n;
            fn mul(self, rhs: $n) -> $n {
                $n::new(self * rhs.$x, self * rhs.$y)
            }
        }

        impl Div for $n {
            type Output = Self;
            fn div(self, rhs: $n) -> Self {
                $n::new(self.$x / rhs.$x, self.$y / rhs.$y)
            }
        }

        impl DivAssign for $n {
            fn div_assign(&mut self, rhs: $n) {
                self.$x /= rhs.$x;
                self.$y /= rhs.$y;
            }
        }

        impl Div<$t> for $n {
            type Output = Self;
            fn div(self, rhs: $t) -> Self {
                $n::new(self.$x / rhs, self.$y / rhs)
            }
        }

        impl DivAssign<$t> for $n {
            fn div_assign(&mut self, rhs: $t) {
                self.$x /= rhs;
                self.$y /= rhs;
            }
        }

        impl Div<$n> for $t {
            type Output = $n;
            fn div(self, rhs: $n) -> $n {
                $n::new(self / rhs.$x, self / rhs.$y)
            }
        }
    };
}

pub(crate) use vec2_impl;

vec2_impl!(vec2, f32, x, y);
vec2_impl!(dvec2, f64, x, y);
