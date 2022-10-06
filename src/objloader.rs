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

        let mut data = common::file_read(path.as_ref())?;
        let mut base = String::new();
        for i in (0..path.as_ref().len()).rev() {
            if path.as_ref().as_bytes()[i] as char == '/' {
                base = path.as_ref()[0..i].to_string();
                break;
            }
        }
        mesh.parse_file(&mut data, base)?;

        mesh.position_count = mesh.positions.len() as u32 / 3;
        mesh.texcoord_count = mesh.texcoords.len() as u32 / 2;
        mesh.normal_count = mesh.normals.len() as u32 / 3;
        mesh.face_count = mesh.face_vertices.len() as u32;
        mesh.index_count = mesh.indices.len() as u32;
        mesh.material_count = mesh.materials.len() as u32;
        mesh.object_count = mesh.objects.len() as u32;
        mesh.group_count = mesh.groups.len() as u32;

        Ok(mesh)
    }

    /*
     * Takes data of line and returns the index of the material,
     * but if the material isn't exsiting, create a new material
     * and returns it's index.
     */
    fn usemtl(&mut self, line: Vec<u8>) -> Result<usize, Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();
        let mtl_name = string[1].to_string();
        /* Find material if it has existed. */
        for (i, m) in self.materials.iter().enumerate() {
            if mtl_name == m.name {
                return Ok(i);
            }
        }
        /* Create a new material with default values if it doesn't found. */
        let mtl = Material::default();
        self.materials.push(mtl);
        Ok(self.materials.len() - 1)
    }

    fn parse_file<T: AsRef<str>>(&mut self, data: &mut Vec<u8>, base: T) -> Result<(), Error>{
        let mut index = 0;
        let mut group_face_count = 0;
        let mut object_face_count = 0;
        let mut current_mtl_index = 0;

        loop {
            let line = data.read_valid_line(&mut index)?;
            if line.len() == 0 {
                break;
            }
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
                    self.face_vertices.push(count);
                    self.face_materials.push(current_mtl_index as u32);
                    group_face_count += 1;
                    object_face_count += 1;
                }
                'g' => {
                    let group = utils::parse_group(line, group_face_count, self.face_vertices.len() as u32, self.indices.len() as u32)?;
                    self.groups.push(group);
                    /* Reset face count to 0 */
                    group_face_count = 0;
                },
                'o' => {
                    let object = utils::parse_object(line, object_face_count, self.face_vertices.len() as u32, self.indices.len() as u32)?;
                    self.objects.push(object);
                    /* Reset face count to 0 */
                    object_face_count = 0;
                }
                'm' => {
                    match (line[1] as char, line[2] as char, line[3] as char, line[4] as char, line[5] as char) {
                        ('t', 'l', 'l', 'i', 'b') => {
                            let mtl = Material::from_obj_line(line, base.as_ref())?;
                            self.materials.push(mtl);
                        }
                        _ => {}
                    }
                }
                'u' => {
                    match (line[1] as char, line[2] as char, line[3] as char, line[4] as char, line[5] as char) {
                        ('s', 'e', 'm', 't', 'l') => {
                            current_mtl_index = self.usemtl(line)?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
