use std::{error::Error, path::Path};

use crate::maths::vec3;

pub struct RawMesh {
    pub vertices: Vec<vec3>,
    pub indices: Vec<u32>,
}

pub fn load_obj<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Vec<RawMesh>, Box<dyn Error>> {
    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
    Ok(models
        .iter()
        .map(|model| {
            let mut positions = vec![vec3::default(); model.mesh.positions.len() / 3];
            for i in 0..model.mesh.positions.len() / 3 {
                positions[i] = vec3::new(
                    model.mesh.positions[3 * i],
                    model.mesh.positions[3 * i + 1],
                    model.mesh.positions[3 * i + 2],
                );
            }
            RawMesh {
                vertices: positions,
                indices: model.mesh.indices.clone(),
            }
        })
        .collect::<Vec<_>>())
}
