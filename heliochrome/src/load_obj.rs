use std::{error::Error, path::Path};

use crate::maths::vec3;

pub struct RawMesh {
    pub vertices: Vec<vec3>,
    pub normals: Vec<vec3>,
    pub indices: Vec<u32>,
}

pub fn load_obj<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Vec<RawMesh>, Box<dyn Error>> {
    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
    Ok(models
        .iter()
        .map(|model| RawMesh {
            vertices: model
                .mesh
                .positions
                .chunks(3)
                .map(|n| vec3::new(n[0], n[1], n[2]))
                .collect(),
            normals: model
                .mesh
                .normals
                .chunks(3)
                .map(|n| vec3::new(n[0], n[1], n[2]))
                .collect(),
            indices: model.mesh.indices.clone(),
        })
        .collect::<Vec<_>>())
}
