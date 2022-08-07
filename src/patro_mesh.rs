use derive_new::new;
use derive_more::{Display, From};
use ndarray::concatenate;
use ndarray::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use num_traits::ToPrimitive;

use crate::patro_mesh_enums::{PatroMeshFormat, PatroCellType};

// cn [ndarray]: coordonnées des noeuds    (nb_nodes x dim)
// co [dict]: connectivités des mailles
// ty [ndarray]: numéro de type de maille
// gno [Group]: groupes de noeuds (dictionnaire de tuples)
// gma [Group]: groupes de mailles (dictionnaire de tuples)
// nodes_name [list]: liste de nom des noeuds
// cells_name [list]: liste de nom de mailles
//     # Setting Properties
// self.cn = cn                    # coordinates
// self.co = co or {}              # connectivity
// self.gno = gno or {}            # groups of nodes
// self.gma = gma or {}            # groups of cells
// self.ty = ty if ty is not None else []      # list of cells' type
// self.ty = np.array(self.ty, dtype=np.uint8)

#[derive(Debug, PartialEq, Clone)]
pub struct PatroMesh {
    // coordonnées des noeuds    (nb_nodes x dim)
    pub cn: Array2<f64>,
    // connectivités des mailles
    pub co: HashMap<u32, Array1<u32>>,
    // ty [ndarray]: numéro de type de maille
    pub ty: Vec<PatroCellType>,
    // gno [Group]: groupes de noeuds (dictionnaire de arrays de numéros de noeuds)
    pub gno: HashMap<String, Vec<u32>>,
    // gma [Group]: groupes de mailles (dictionnaire de arrays de numéros de mailles)
    pub gma: HashMap<String, Vec<u32>>,
    // nodes_name : liste de nom des noeuds
    pub nodes_name: Vec<String>,
    // cells_name : list de nom de mailles
    pub cells_name: Vec<String>,
}

impl PatroMesh {
    pub fn new() -> Self {
        PatroMesh {
            cn: Array2::<f64>::zeros([0, 3]),
            co: HashMap::new(),
            ty: vec![],
            gno: HashMap::new(),
            gma: HashMap::new(),
            nodes_name: vec![],
            cells_name: vec![],
        }
    }

    pub fn add_nodes(&mut self, nodes: &Array2<f64>, nodes_name: &Vec<String>) {
        self.cn = concatenate![Axis(0), self.cn, nodes.clone()];
        self.nodes_name.extend(nodes_name.clone());
    }

    pub fn add_cells(&mut self, connectivities: &Vec<Array1<u32>>, ty: PatroCellType)  -> Result<Vec<u32>, &str>{
        match ty {
            PatroCellType::SEG2 => {
                return self.add_seg2_cells(connectivities);
            },
            _ => {
                return Err("Cell type not supported");
            }
        }
    }
    pub fn add_seg2_cells(&mut self, connectivities: &Vec<Array1<u32>>, ) -> Result<Vec<u32>, &str> {
        let mut cells = vec![];
        for connectivity in connectivities.iter() {
            let cell_id = self.co.len().to_u32().unwrap();
            self.co.insert(cell_id.to_owned(), connectivity.clone());
            cells.push(cell_id);
            self.ty.push(PatroCellType::SEG2);
            self.cells_name.push(format!("M{}", cell_id + 1));
        }
        return Ok(cells);
    }

    pub fn read_mesh(filename: &str, format: PatroMeshFormat) -> PatroMesh {
        let mut mesh = PatroMesh::new();
        match format {
            PatroMeshFormat::Mail => mesh.read_mail_format(filename),
        }
        return mesh;
    }
    pub fn read_mail_format(&self, filename: &str) {
        println!("Reading file {}", filename);

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        // TODO: parse contents with regex

        println!("Ended reading {}", filename);
    }
}
