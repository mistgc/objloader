#[allow(dead_code)]
#[allow(unused)]

use crate::utils::{ PATH_SEPARATOR, MoreStrMethod };
use crate::common;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Texture {
    /* Texture name from .mtl file */
    name:                   String,

    /* Resolved path to texture */
    path:                   String,
}

impl Texture {
    pub fn new<T: AsRef<str>>(name: T, path: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            path: path.as_ref().to_string(),
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct Material {
    /* Material name */
    pub name:                   String,

    /* Parameters */
    Ka:                     [f32; 3],       /* Ambient */
    Kd:                     [f32; 3],       /* Diffuse */
    Ks:                     [f32; 3],       /* Specular */
    Ke:                     [f32; 3],       /* Emission */
    Kt:                     [f32; 3],       /* Transmittance */
    Ns:                     f32,            /* Shininess */
    Ni:                     f32,            /* Index of refraction */
    Tf:                     [f32; 3],       /* Transmission filter */
    d:                      f32,            /* Dissolve (alpha) */
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

#[allow(non_snake_case)]
impl Material {
    fn parse_single(line: Vec<u8>) -> Result<f32, Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();
        let result = string[1].parse()?;
        Ok(result)
    }

    fn parse_triple(line: Vec<u8>) -> Result<[f32; 3], Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();
        let mut result = [0.; 3];
        result[0] = string[1].parse()?;
        result[1] = string[2].parse()?;
        result[2] = string[3].parse()?;
        Ok(result)
    }

    fn parse_int(line: Vec<u8>) -> Result<i32, Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();
        let result = string[1].parse()?;
        Ok(result)
    }
    
    fn parse_map(line: Vec<u8>, base: &str) -> Result<Texture, Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();
        let name = string[1].to_string();
        let path: String = base.to_owned() + PATH_SEPARATOR + &name;

        Ok(Texture::new(name, path))
    }

    /*
     * Takes data of line and the base (parent) path and returns
     * a new material.
     */
    pub fn from_obj_line<T: AsRef<str>>(line: Vec<u8>, base: T) -> Result<Self, Error> {
        let strings = String::from_utf8(line)?;
        let string: Vec<&str> = strings.split(' ').collect();

        /* Load the .mtl file. */
        let data = common::file_read(base.as_ref().to_string() + PATH_SEPARATOR + string[1])?;

        let mut index = 0;
        let mut mtl = Self::default();
        let mut found_d = false;
        
        /* Continuously parse the file line by line. */
        loop {
            let line = data.read_valid_line(&mut index)?;
            if line.len() == 0 {
                break;
            }
            match line[0] as char {
                'n' => {
                    match (line[1] as char, line[2] as char, line[3] as char, line[4] as char, line[5] as char) {
                        ('e', 'w', 'm', 't', 'l') => {
                            let strings = String::from_utf8(line)?;
                            let string: Vec<&str> = strings.split(' ').collect();
                            mtl.name = string[1].to_string();
                        }
                        _ => {}
                    }
                }
                'K' => {
                    match line[1] as char {
                        'a' => mtl.Ka = Self::parse_triple(line)?,
                        'd' => mtl.Kd = Self::parse_triple(line)?,
                        's' => mtl.Ks = Self::parse_triple(line)?,
                        'e' => mtl.Ke = Self::parse_triple(line)?,
                        't' => mtl.Kt = Self::parse_triple(line)?,
                         _  => {}
                    }
                }
                'N' => {
                    match line[1] as char {
                        's' => mtl.Ns = Self::parse_single(line)?,
                        'i' => mtl.Ni = Self::parse_single(line)?,
                         _  => {}
                    }
                }
                'T' => {
                    match line[1] as char {
                        'r' => {
                            let Tr = Self::parse_single(line)?;
                            /* If found the d, we could ignore the Tr. */
                            if !found_d {
                                mtl.d = 1. - Tr;
                            }
                        }
                        'f' => mtl.Tf = Self::parse_triple(line)?,
                         _  => {}
                    }
                }
                'd' => {
                    if (line[1] as char).is_whitespace() {
                        mtl.d = Self::parse_single(line)?;
                    }
                }
                'i' => {
                    match (line[1] as char, line[2] as char, line[3] as char, line[4] as char) {
                        ('l', 'l', 'u', 'm') => mtl.illum = Self::parse_int(line)?,
                        _ => {}
                    }
                }
                'm' => {
                    match (line[1] as char, line[2] as char, line[3] as char) {
                        ('a', 'p', '_') => {
                            match line[4] as char {
                                'K'       => {
                                    match line[5] as char {
                                        'a' => mtl.map_Ka = Self::parse_map(line, base.as_ref())?,
                                        'd' => mtl.map_Kd = Self::parse_map(line, base.as_ref())?,
                                        's' => mtl.map_Ks = Self::parse_map(line, base.as_ref())?,
                                        'e' => mtl.map_Ke = Self::parse_map(line, base.as_ref())?,
                                        't' => mtl.map_Kt = Self::parse_map(line, base.as_ref())?,
                                         _  => {}
                                    }
                                }
                                'N'       => {
                                    match line[5] as char {
                                        's' => mtl.map_Ns = Self::parse_map(line, base.as_ref())?,
                                        'i' => mtl.map_Ni = Self::parse_map(line, base.as_ref())?,
                                         _  => {}
                                    }
                                }
                                'd'       => {
                                    mtl.map_d = Self::parse_map(line, base.as_ref())?;
                                    found_d = true;
                                }
                                'b' | 'B' => {
                                    match (line[5] as char, line[6] as char, line[7] as char) {
                                        ('u', 'm', 'p') => mtl.map_bump = Self::parse_map(line, base.as_ref())?,
                                        _               => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                 _  => {}
            }
        }

        Ok(mtl)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name:           String::new(),
            Ka:             [0., 0., 0.],
            Kd:             [1., 1., 1.],
            Ke:             [0., 0., 0.],
            Ks:             [0., 0., 0.],
            Kt:             [0., 0., 0.],
            Ns:             1.,
            Ni:             1.,
            Tf:             [1., 1., 1.],
            d:              1.,
            illum:          1,

            map_Ka:         Texture::default(),
            map_Kd:         Texture::default(),
            map_Ks:         Texture::default(),
            map_Ke:         Texture::default(),
            map_Kt:         Texture::default(),
            map_Ns:         Texture::default(),
            map_Ni:         Texture::default(),
            map_d:          Texture::default(),
            map_bump:       Texture::default(),
        }
    }
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
