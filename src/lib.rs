extern crate core;

mod mesh;
mod mesh_enums;
mod node;
mod cell;
mod parsers;

pub mod lib{
    pub use crate::mesh::Mesh;
    pub use crate::mesh_enums::{CellType, MeshFormat};
    pub use crate::node::Node;
    pub use crate::cell::MeshCell;
    pub use crate::parsers::mail_parser;
}

#[cfg(test)]
mod tests {


    // #[test]
    // fn read_mail_file_should_work() {
    //     todo!();
    //     let mail_text: &str = "
    // --------------------------------------------------------------------------------
    //  TITRE
    // MA-02-JUIN-2020 11:22:47
    //  FINSF
    //  %
    //  COOR_3D
    //  N1        1.00000000000000E+00  4.00000000000000E+00  2.50000000000000E+00
    //  N2        2.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    //  N3        3.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    //  N4        4.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    // FINSF
    //  %
    // %
    //  POI1
    //  M1       N1
    //  M2       N3
    //  M3       N2
    //  M4       N4
    //  FINSF
    //  %
    //  SEG2
    //  M5       N2       N1
    //  M6       N3       N2
    //  M7       N4       N3
    // FINSF
    //
    //   %
    //  GROUP_MA
    //  APPUI
    //  M1       M2       M3       M4
    //  FINSF
    //   %
    //  GROUP_NO
    //  NOEU_MO
    //  N1       N2       N3       N4
    //   FINSF
    //   %
    //  FIN
    //                 ";
    //     let path = Path::new(".\\src\\data\\test_meshfile.mail");
    //     let display = path.display();
    //
    //     // Open a file in write-only mode, returns `io::Result<File>`
    //     let mut file = match File::create(&path) {
    //         Err(why) => panic!("couldn't create {}: {}", display, why),
    //         Ok(file) => file,
    //     };
    //
    //     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    //     match file.write_all(mail_text.as_bytes()) {
    //         Err(why) => panic!("couldn't write to {}: {}", display, why),
    //         Ok(_) => println!("successfully wrote to {}", display),
    //     }
    //     // FIXME faire vrai test
    //     assert_eq!(false, true);
    // }
}
