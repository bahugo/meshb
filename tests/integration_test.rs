use std::path::PathBuf;

use meshb::lib::{MeshFormat, Mesh, Node, MeshCell};
#[test]
fn test_mesh_from_mail_file() {
    let mesh_file: PathBuf = [
        env!("CARGO_MANIFEST_DIR"), "tests", "resources", "mesh_1.mail"]
        .iter()
        .collect();

    let mesh_res = Mesh::read_mesh(mesh_file, MeshFormat::Mail);
    //
    // panic!("{:#?}", mesh);
    assert_eq!(mesh_res.is_ok(), true);
    let mesh = mesh_res.unwrap();
    assert_eq!(mesh.get_cell_co(3).unwrap(), vec![3]);
    assert_eq!(*(mesh.nodes.get(&3).unwrap()), Node{x: 4.00000000000000E+00, y: 4.00000000000000E+00, z: 1.50000000000000E+00, } );
    assert_eq!(*(mesh.cells.get(&3).unwrap()), MeshCell{ ty:meshb::lib::CellType::POI1, co: vec![3] } );
}
