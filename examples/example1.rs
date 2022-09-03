use objloader::Mesh;

fn main() {
    let mesh = Mesh::from_file("test_cube/test_cube.obj").unwrap();
    println!("{:?}", mesh);
}
