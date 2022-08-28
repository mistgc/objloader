#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::common;

pub struct Texture {
    /* Texture name from .mtl file */
    name:                   String,

    /* Resolved path to texture */
    path:                   String,
}

pub struct Material {
    /* Material name */
    name:                   String,

    /* Parameters */
    Ka:                     [f32; 3],       /* Ambient */
    Kd:                     [f32; 3],       /* Diffuse */
    Ks:                     [f32; 3],       /* Specular */
    Ke:                     [f32; 3],       /* Emission */
    Kt:                     [f32; 3],       /* Transmittance */
    Ns:                     f32,            /* Shininess */
    Ni:                     f32,            /* Index of refraction */
    Tf:                     [f32; 3],       /* Transmission filter */
    d:                      f32,            /* Disolve (alpha) */
    illum:                  i32,            /* Illumination model */

    /* Texture maps */
    map_Ka:                 Texture,
    map_Kd:                 Texture,
    map_Ks:                 Texture,
    map_Ke:                 Texture,
    map_Kt:                 Texture,
    map_Ns:                 Texture,
    map_Ni:                 Texture,
    map_d:                  Texture,
    map_bump:               Texture,
}

pub struct Index {
    p:                      u32,
    t:                      u32,
    n:                      u32,
}

pub struct Group {
    /* Group name */
    name:                   String,

    /* Number of faces */
    face_count:             u32,
    
    /* First face in Mesh face_* arrays */
    face_offset:            u32,

    /* First index in Mesh indices array */
    index_offset:           u32,
}

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
    face_vertices:          Vec<u32>,
    face_materials:         Vec<u32>,

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
