use std::{error::Error, path::Path};

use crate::maths::vec3;

pub fn load_obj<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<(Vec<vec3>, Vec<u32>), Box<dyn Error>> {
    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

    //TODO: rn it only loads the first model lol.
    for model in models {
        let mut positions = vec![vec3::default(); model.mesh.positions.len() / 3];
        for i in 0..model.mesh.positions.len() / 3 {
            positions[i] = vec3::new(
                model.mesh.positions[3 * i + 0],
                model.mesh.positions[3 * i + 1],
                model.mesh.positions[3 * i + 2],
            );
        }
        return Ok((positions, model.mesh.indices));
    }

    Err("could not load :<")?
}
