use crate::utils::{ self, MoreStrMethod };
use crate::raw;

#[test]
fn read_line() {
    let lines = r"Hello world!!!
Test Test Test...



123456...



!!!
aaa
bbb"
    .to_string();
    let mut index = 0;
    let line = String::from_utf8(lines.as_bytes().to_vec().read_line(&mut index).unwrap()).unwrap();
    assert_eq!("Hello world!!!", line);
    let line = String::from_utf8(lines.as_bytes().to_vec().read_line(&mut index).unwrap()).unwrap();
    assert_eq!("Test Test Test...", line);
    let line = String::from_utf8(lines.as_bytes().to_vec().read_valid_line(&mut index).unwrap()).unwrap();
    assert_eq!("123456...", line);
    let line = String::from_utf8(lines.as_bytes().to_vec().read_valid_line(&mut index).unwrap()).unwrap();
    assert_eq!("!!!", line);
    let line = String::from_utf8(lines.as_bytes().to_vec().read_valid_line(&mut index).unwrap()).unwrap();
    assert_eq!("aaa", line);
    let line = String::from_utf8(lines.as_bytes().to_vec().read_valid_line(&mut index).unwrap()).unwrap();
    assert_eq!("bbb", line);
}

#[test]
fn parse_position() {
    let line: Vec<u8> = "v 1.0 2.0 3.0".try_into().unwrap();
    let vertex = utils::parse_vertex(line).unwrap();
    assert_eq!(vec![1.0, 2.0, 3.0], vertex);

    let line: Vec<u8> = "vn 2.0 3.0 4.0".try_into().unwrap();
    let vertex = utils::parse_vertex(line).unwrap();
    assert_eq!(vec![2.0, 3.0, 4.0], vertex);
    
    let line: Vec<u8> = "vt 3.0 4.0 5.0".try_into().unwrap();
    let vertex = utils::parse_vertex(line).unwrap();
    assert_eq!(vec![3.0, 4.0, 5.0], vertex);
}

#[test]
fn parse_face() {
    let line: Vec<u8> = "f 1/1/1 2/2/2 3/3/3".try_into().unwrap();
    let result = utils::parse_face(line).unwrap();
    assert_eq!(raw::Index::new(1, 1, 1), result.0[0]);
    assert_eq!(raw::Index::new(2, 2, 2), result.0[1]);
    assert_eq!(raw::Index::new(3, 3, 3), result.0[2]);
    assert_eq!(3, result.1);

    let line: Vec<u8> = "f 1/1 2/2 3/3 4/4".try_into().unwrap();
    let result = utils::parse_face(line).unwrap();
    assert_eq!(raw::Index::new(1, 1, 0), result.0[0]);
    assert_eq!(raw::Index::new(2, 2, 0), result.0[1]);
    assert_eq!(raw::Index::new(3, 3, 0), result.0[2]);
    assert_eq!(raw::Index::new(4, 4, 0), result.0[3]);
    assert_eq!(4, result.1);
}
