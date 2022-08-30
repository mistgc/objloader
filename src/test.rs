use crate::utils::{ self, MoreStrMethod };

#[test]
fn read_line() {
    let lines = r"Hello world!!!
Test Test Test...



123456...



!!!"
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
