#![feature(map_first_last)]
#![feature(portable_simd)]
#![feature(const_trait_impl)]
#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
// lib but not and maths
#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub mod load_obj;

pub mod accel;
pub mod camera;
pub mod color;
pub mod context;
pub mod hittables;
pub mod image;
pub mod materials;
pub mod maths;
pub mod object;
pub mod pdf;
pub mod scene;
pub mod sdf;
pub mod tonemap;
pub mod transform;
pub mod util;
