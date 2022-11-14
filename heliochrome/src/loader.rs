use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{context::QualitySettings, maths::vec2, scene::Scene, tonemap::ToneMap};

pub trait FromHCY: Sized {
    fn from_hcy(member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct ImageConfig {
    pub size: vec2,
    pub quality: QualitySettings,
    pub tone_map: ToneMap,
}

pub fn collect_until_next_item<'a>(
    line_iter: &mut (impl Iterator<Item = &'a String> + Clone),
) -> Vec<String> {
    let mut peek_iter = line_iter.clone().peekable();
    if let Some(first_str) = peek_iter.peek() {
        let prefix = first_str
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let vec = peek_iter
            .take_while(|x| x.trim().is_empty() || x.starts_with(&prefix))
            .filter(|x| !x.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>();

        // we already know it will be able to advance this many times, just to silence the warning
        let _ = line_iter.advance_by(vec.len());

        vec
    } else {
        vec![]
    }
}

pub fn parse_into<T: std::str::FromStr>(value: &str) -> Result<T, Box<dyn Error>>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    Ok(value
        .trim()
        .parse::<T>()
        .map_err(|err| format!("could not parse key, {err} ({value})"))?)
}

impl FromHCY for ImageConfig {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut width = None;
        let mut height = None;
        let mut bounces = None;
        let mut samples = None;
        let mut tone_map = None;

        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "width" => width = Some(parse_into::<u32>(value)?),
                "height" => height = Some(parse_into::<u32>(value)?),
                "bounces" => bounces = Some(parse_into(value)?),
                "samples" => samples = Some(parse_into(value)?),
                "tone map" => {
                    tone_map = Some(
                        ToneMap::from_hcy(Some(value), collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse tone map key: {err}"))?,
                    );
                }
                _ => {}
            }
        }

        Ok(ImageConfig {
            size: vec2::new(
                width.ok_or("could not find required key width.")? as f32,
                height.ok_or("could not find required key height.")? as f32,
            ),
            quality: QualitySettings {
                bounces: bounces.ok_or("could not find required key bounces.")?,
                samples: samples.ok_or("could not find required key samples.")?,
            },
            tone_map: tone_map.ok_or("coult not find required key `tone map`.")?,
        })
    }
}

#[derive(Debug)]
pub struct SceneConfig {
    pub image_config: ImageConfig,
    pub scene: Scene,
}

impl FromHCY for SceneConfig {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut image_config = None;
        let mut scene = None;

        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            match line.trim() {
                "image:" => {
                    image_config = Some(
                        ImageConfig::from_hcy(None, collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse image key: {err}"))?,
                    );
                }
                "scene:" => {
                    scene = Some(
                        Scene::from_hcy(None, collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse scene key: {err}"))?,
                    );
                }
                _ => {}
            }
        }

        let image_config = image_config.ok_or("could not find required key `image`.")?;
        let mut scene = scene.ok_or("could not find required key `scene`.")?;
        scene.camera.size = image_config.size;
        Ok(SceneConfig {
            image_config,
            scene,
        })
    }
}

pub fn load_scene_file(path: &Path) -> Result<SceneConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let raw_line_iter = reader.lines().map(|x| x.unwrap());
    let lines = raw_line_iter.collect();
    SceneConfig::from_hcy(None, lines)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn load() {
        dbg!(load_scene_file(Path::new("C:/dev/heliochrome/assets/scene/basic.hcy")).unwrap());
        println!("hi");
    }
}
