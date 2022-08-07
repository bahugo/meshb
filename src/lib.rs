mod patro_mesh;
mod patro_mesh_enums;

#[cfg(test)]
mod tests {
    use std::env::{current_dir, join_paths};
    use ndarray;
    use ndarray::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::ops::Index;
    use std::path::Path;
    use num_traits::cast::ToPrimitive;
    use crate::patro_mesh::PatroMesh;
    use crate::patro_mesh_enums::PatroCellType;

    #[test]
    fn patro_mesh_init_empty_should_work() {
        let mesh = PatroMesh::new();
        assert_eq!(mesh.cn.shape(), [0, 3]);
        assert_eq!(mesh.co.len(), 0);
        assert_eq!(mesh.ty.len(), 0);
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
        assert_eq!(mesh.nodes_name.len(), 0);
        assert_eq!(mesh.cells_name.len(), 0);
    }
    #[test]
    fn patro_mesh_add_nodes_should_work() {
        let mut mesh = PatroMesh::new();
        let nodes_names = vec!["N12".to_owned(), "N2".to_owned()];
        let nodes = array![[3., 0., 1.], [2., 1., 1.]];
        mesh.add_nodes(&nodes, &nodes_names);
        assert_eq!(mesh.cn.shape(), [2, 3]);
        assert_eq!(mesh.cn.slice(s![0, ..]), array![3., 0., 1.]);
        assert_eq!(mesh.cn.slice(s![1, ..]), array![2., 1., 1.]);
        assert_eq!(mesh.nodes_name.len(), 2);
        assert_eq!(mesh.nodes_name[0], "N12");
        assert_eq!(mesh.nodes_name[1], "N2");
        assert_eq!(mesh.co.len(), 0);
        assert_eq!(mesh.ty.len(), 0);
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
        assert_eq!(mesh.cells_name.len(), 0);
        let nodes_names = vec!["N21".to_owned(), "N222".to_owned()];
        let nodes = array![[3.2, 0.3, 1.3], [2.1, 1.1, 1.1]];
        mesh.add_nodes(&nodes, &nodes_names);
        assert_eq!(mesh.cn.shape(), [4, 3]);
        assert_eq!(mesh.cn.slice(s![2, ..]), array![3.2, 0.3, 1.3]);
        assert_eq!(mesh.cn.slice(s![3, ..]), array![2.1, 1.1, 1.1]);
        assert_eq!(mesh.nodes_name.len(), 4);
        assert_eq!(mesh.nodes_name[2], "N21");
        assert_eq!(mesh.nodes_name[3], "N222");
    }
    #[test]
    fn patro_mesh_add_cells_should_work() {
        let mut mesh = PatroMesh::new();
        let nodes_names = vec!["N1".to_owned(), "N2".to_owned(), "N3".to_owned(), "N4".to_owned()];
        let nodes = array![[3., 0., 1.], [2., 1., 1.], [3.2, 0.3, 1.3], [2.1, 1.1, 1.1]];
        mesh.add_nodes(&nodes, &nodes_names);
        // 2 cells
        let nb_cells = mesh.co.len();
        let new_cells = mesh.add_cells(&vec![array![1_u32, 2_u32], array![3_u32, 4_u32]],
                                        PatroCellType::SEG2);
        // assert_eq!(new_cells, Array::range(nb_cells, nb_cells+2, 1);
        assert_eq!(mesh.co.len(), nb_cells + 2);
        let nb_cells_u32 = nb_cells.to_u32().to_owned().unwrap();
        assert_eq!(mesh.co.keys().max().unwrap().to_owned(), nb_cells_u32 + 1_u32);
        assert_eq!(mesh.ty.len(), nb_cells + 2);
        assert_eq!(mesh.cells_name.len(), nb_cells + 2);
        //
        // # one cell
        // nb_cells = len(self.mesh.co)
        // self.mesh.add_cells([[3, 4]], ty=PATRO_SEG2)
        // self.assertEqual(len(self.mesh.co), nb_cells + 1)
        // self.assertEqual(len(self.mesh.ty), nb_cells + 1)
        // self.assertEqual(len(self.mesh.cells_name), nb_cells + 1)
        // # add cells on a empty mesh
        // self.mesh.co = dict()
        // self.mesh.ty = np.array([])
        // self.mesh.add_cells([[3, 4]], ty=PATRO_SEG2)
        // self.assertEqual(len(self.mesh.co), 1)
        // self.assertEqual(len(self.mesh.ty), 1)
        // self.assertEqual(len(self.mesh.cells_name), 1)
        // # add cells and deduce type
        // self.mesh.co = dict()
        // self.mesh.add_cells([[1], [3, 4], [1, 2, 3], [1, 2, 3, 4],])
        // self.assertEqual(self.mesh.ty.tolist(), [PATRO_POI1, PATRO_SEG2, PATRO_TRIA3, PATRO_QUAD4])
    }

    #[test]
    fn read_mail_file_should_work() {

        let mail_text: &str = "
    --------------------------------------------------------------------------------
     TITRE
    MA-02-JUIN-2020 11:22:47
     FINSF
     %
     COOR_3D
     N1        1.00000000000000E+00  4.00000000000000E+00  2.50000000000000E+00
     N2        2.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
     N3        3.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
     N4        4.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    FINSF
     %
    %
     POI1
     M1       N1
     M2       N3
     M3       N2
     M4       N4
     FINSF
     %
     SEG2
     M5       N2       N1
     M6       N3       N2
     M7       N4       N3
    FINSF

      %
     GROUP_MA
     APPUI
     M1       M2       M3       M4
     FINSF
      %
     GROUP_NO
     NOEU_MO
     N1       N2       N3       N4
      FINSF
      %
     FIN
                    ";
        let path = Path::new(".\\src\\data\\test_meshfile.mail");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(mail_text.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        // FIXME faire vrai test
        assert_eq!(false, true);
    }
}
