use std::{
    error::Error,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use heliochrome::{
    context::Context, image::Image, loader::load_scene_file, tonemap::ToneMap, util::write_image,
};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    scene_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = load_scene_file(Path::new(&args.scene_file))?;
    let context = Context::new(
        config.image_config.size,
        config.image_config.quality,
        config.scene,
        config.image_config.tone_map,
    );

    context.start_full_render();
    let pixel_receiver = context.pixel_receiver;
    let mut out_image = Image::new(config.image_config.size);
    let pb = ProgressBar::new(config.image_config.size.prod() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner} [{elapsed_precise}] {wide_bar}")
            .unwrap()
            .tick_chars("🌕🌖🌗🌘🌑🌒🌓🌔🌕"),
    );

    loop {
        let (color, uv) = pixel_receiver.recv()?;
        out_image.set_pixel(&uv, color);
        pb.inc(1);

        if pb.position() == config.image_config.size.prod() as u64 {
            break;
        }
    }

    pb.finish();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let file = format!("img_{}.png", now.as_secs());
    let path = Path::new(&file);
    context.tone_map.map(&mut out_image);
    write_image(
        path,
        out_image.size,
        if matches!(context.tone_map, ToneMap::HejlRichard) {
            1.0
        } else {
            2.2
        },
        &out_image,
    )?;

    println!("successfully wrote render to {:?}", path);

    Ok(())
}
