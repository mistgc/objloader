#[allow(dead_code)]

use crate::error::Error;
use crate::raw::*;

#[cfg(not(target_os = "windows"))]
pub const PATH_SEPARATOR: &str = "/";

#[cfg(target_os = "windows")]
pub const PATH_SEPARATOR: &str = "\\";

pub trait MoreCharMethod {
    fn is_end_of_name(&self) -> bool;

    fn is_newline(&self) -> bool;

    fn is_exponent(&self) -> bool;
}

pub trait MoreStrMethod {
    fn skip_whitespace(&self, index: &mut usize) -> Result<(), Error>;

    fn skip_line(&self, index: &mut usize) -> Result<(), Error>;

    fn read_line(&self, index: &mut usize) -> Result<Vec<u8>, Error>;

    fn read_valid_line(&self, index: &mut usize) -> Result<Vec<u8>, Error>;
}

impl MoreCharMethod for char {
    #[inline]
    fn is_newline(&self) -> bool {
        match *self {
            '\n' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_exponent(&self) -> bool {
        match *self {
            'e' | 'E' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_end_of_name(&self) -> bool {
        match *self {
            '\t' | '\r' | '\n' => true,
            _ => false,
        }
    }
}

impl MoreStrMethod for Vec<u8> {
    /* 
     * Skips the whitespaces in the text data.
     * If index greeter 'self.len()', returns Err(Error::IndexOutOfBound),
     * otherwise returns Ok(()).
     */
    #[inline]
    fn skip_whitespace(&self, index: &mut usize) -> Result<(), Error> {
        let length = self.len();
        if *index >= length {
            return Err(Error::IndexOutOfBound);
        }
        while <u8 as Into<char>>::into(self[*index]).is_whitespace() && *index < length {
            *index += 1;
        }

        Ok(())
    }

    /*
     * Skips the line in the text data.
     * If index greeter 'self.len()', returns Err(Error::IndexOutOfBound),
     * otherwise returns Ok(()).
     */
    #[inline]
    fn skip_line(&self, index: &mut usize) -> Result<(), Error> {
        let length = self.len();
        if *index == length {
            return Err(Error::IndexOutOfBound);
        }

        while *index < length && <u8 as Into<char>>::into(self[*index]).is_newline() {
            *index += 1;
        }

        Ok(())
    }

    /*
     * Returns the data of a line from the .obj/.mtl file.
     * The param 'index' is a state recording the current line needed reading.
     * And returns the data of a line whitin Ok(Vec<u8>),
     * otherwise returns Err(Error::IndexOutOfBound).
     */
    #[inline]
    fn read_line(&self, index: &mut usize) -> Result<Vec<u8>, Error> {
        let length = self.len();
        if *index >= length {
            return Err(Error::IndexOutOfBound);
        }

        let mut line = vec![];

        while *index < length && !(self[*index] as char).is_newline() {
            line.push(self[*index]);
            if self[*index] as char == '\0' {
                return Ok(line);
            }
            *index += 1;
        }

        *index += 1;    // skip '\n' and enter next line

        Ok(line)
    }

    /*
     * Returns data of a line has text.
     * It will skip all continuous '\n'.
     * The param 'index' is a state recording the current line needed reading.
     * And Returns data of the valid line whitin Ok(Vec<u8>),
     * otherwise returns Err(Error::IndexOutOfBound).
     */
    #[inline]
    fn read_valid_line(&self, index: &mut usize) -> Result<Vec<u8>, Error> {
        let length = self.len();
        if *index >= length {
            return Err(Error::IndexOutOfBound);
        }

        let mut line = vec![];

        self.skip_line(index)?;

        while *index < length && !(self[*index] as char).is_newline() {
            line.push(self[*index]);
            *index += 1;
        }

        Ok(line)
    }
}

/*
 * Takes data of line and returns the Vec data '[x, y, z]' for the vertex.
 */
pub fn parse_vertex(line: Vec<u8>) -> Result<Vec<f32>, Error> {
    let mut vertex = vec![];
    let strings = String::from_utf8(line.clone())?;
    let string: Vec<&str> = strings.split(' ').collect();
    for i in 1..string.len() {
        vertex.push(string[i].parse()?);
    }
    Ok(vertex)
}

/*
 * Takes data of line and returns Vec<Index> data and the number of indices.
 * Like:
 *      ([Index(1, 1, 1), Index(2, 2, 2), Index(3, 3, 3)], 3)
 */
pub fn parse_face(line: Vec<u8>) -> Result<(Vec<Index>, u32), Error> {
    let mut indices = vec![];
    let strings = String::from_utf8(line.clone())?;
    let string: Vec<&str> = strings.split(' ').collect();
    let (mut v, mut t, mut n) = (0, 0, 0);
    for i in 1..string.len() {
        let components: Vec<&str> = string[i].split('/').collect();
        match components.len() {
            1 => v = components[0].parse()?,
            2 => { v = components[0].parse()?; t = components[1].parse()?; },
            3 => { v = components[0].parse()?; t = components[1].parse()?; n = components[2].parse()?; },
            _ => return Err(Error::InvalidComponents),
        }

        indices.push(Index::new(v, t, n));
    }

    Ok((indices, string.len() as u32 - 1))
}

pub fn parse_group(line: Vec<u8>, face_count: u32, face_offset: u32, index_offset: u32) -> Result<Group, Error> {
    let strings = String::from_utf8(line.clone())?;
    let string: Vec<&str> = strings.split(' ').collect();
    Ok(Group::new(string[1].to_string(), face_count, face_offset, index_offset))
}

pub fn parse_object(line: Vec<u8>, face_count: u32, face_offset: u32, index_offset: u32) -> Result<Group, Error> {
    let strings = String::from_utf8(line.clone())?;
    let string: Vec<&str> = strings.split(' ').collect();
    Ok(Group::new(string[1].to_string(), face_count, face_offset, index_offset))
}

pub fn parse_usemtl() {
}
