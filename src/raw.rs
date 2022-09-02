#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]

#[derive(Debug, PartialEq, Eq)]
pub struct Texture {
    /* Texture name from .mtl file */
    name:                   String,

    /* Resolved path to texture */
    path:                   String,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Index {
    v:                      u32,
    t:                      u32,
    n:                      u32,
}

impl Index {
    pub fn new(v: u32, t: u32, n: u32) -> Self {
        Self { v, t, n }
    }
}

impl Default for Index {
    fn default() -> Self {
        Self { v: 0, t: 0, n: 0 }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl Group {
    pub fn new(name: String, face_count: u32, face_offset: u32, index_offset: u32) -> Self {
        Self {
            name,
            face_count,
            face_offset,
            index_offset
        }
    }
}
