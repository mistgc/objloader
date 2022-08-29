use crate::util::{ self, MoreStrMethod };

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
