#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::common;
use crate::raw::*;

pub struct Mesh {
    /* Vertex data */
    position_count:         u32,
    positions:              Vec<f32>,

    texcoord_count:         u32,
    texcoords:              Vec<f32>,

    normal_count:           u32,
    normals:                Vec<f32>,

    /* Face data: one element for each face */
    face_count:             u32,
    face_vertices:          Vec<u32>,           /* store amount of vertices of each face */
    face_materials:         Vec<u32>,           /* store the material of each face */

    /* Index data: one element for each face vertex */
    index_count:            u32,
    indices:                Vec<Index>,

    /* Materials */
    material_count:         u32,
    materials:              Vec<Material>,

    /* Mesh objects ('0' tag in .obj file) */
    object_count:           u32,
    objects:                Vec<Group>,

    /* Mesh groups ('g' tag in .obj file) */
    group_count:            u32,
    groups:                 Vec<Group>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            position_count: 0,
            positions: vec![],
            texcoord_count: 0,
            texcoords: vec![],
            normal_count: 0,
            normals: vec![],
            face_count: 0,
            face_vertices: vec![],
            face_materials: vec![],
            index_count: 0,
            indices: vec![],
            material_count: 0,
            materials: vec![],
            object_count: 0,
            objects: vec![],
            group_count: 0,
            groups: vec![],
        }
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file() -> Self {
        let mesh = Self::new();

        // TODO: load data from file

        mesh
    }
}
