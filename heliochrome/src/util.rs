use std::path::Path;

use image::{ImageBuffer, Rgba};

use crate::{image::Image, maths::vec2};

pub fn write_image(
    path: &Path,
    size: vec2,
    gamma: f32,
    img: &Image,
) -> Result<(), Box<dyn std::error::Error>> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
        size.x as u32,
        size.y as u32,
        img.to_gamma_corrected_rgba8(gamma).to_vec(),
    )
    .ok_or("oof")?;

    let mut pb = path.to_path_buf();
    pb.set_extension("png");
    img.save_with_format(pb.as_path(), image::ImageFormat::Png)?;

    Ok(())
}
