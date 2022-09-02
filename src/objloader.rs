#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::common;
use crate::raw::*;
use crate::error::Error;
use crate::utils;
use crate::utils::MoreStrMethod;

#[derive(Debug)]
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

    pub fn from_file<T: AsRef<str>>(path: T) -> Result<Self, Error> {
        let mut mesh = Self::new();
        /* Add dummy data because index of vertex is beginning by 1 instead of 0. */
        mesh.positions.push(0.);
        mesh.positions.push(0.);
        mesh.positions.push(0.);

        mesh.texcoords.push(0.);
        mesh.texcoords.push(0.);

        mesh.normals.push(0.);
        mesh.normals.push(0.);
        mesh.normals.push(1.);

        let mut data = common::file_read(path)?;

        // TODO: load data from file

        mesh.position_count = mesh.positions.len() as u32 / 3;
        mesh.texcoord_count = mesh.texcoords.len() as u32 / 2;
        mesh.normal_count = mesh.normals.len() as u32 / 3;

        Ok(mesh)
    }

    fn parse_file(&mut self, data: &mut Vec<u8>) -> Result<(), Error>{
        let mut index = 0;

        loop {
            let line = data.read_line(&mut index)?;
            match line[0] as char {
                'v' => {
                    match line[1] as char {
                        ' ' => {
                            let vertices = utils::parse_vertex(line)?;
                            for v in vertices {
                                self.positions.push(v);
                            }
                        },
                        't' => {
                            let vertices = utils::parse_vertex(line)?;
                            for v in vertices {
                                self.texcoords.push(v);
                            }
                        }
                        'n' => {
                            let vertices = utils::parse_vertex(line)?;
                            for v in vertices {
                                self.normals.push(v);
                            }
                        }
                        _ => {}
                    }
                }
                'f' => {
                    let (indices, count) = utils::parse_face(line)?;
                    for index in indices {
                        self.indices.push(index);
                    }
                    self.index_count += count;
                }
                _ => {}
            }
        }
    }
}
