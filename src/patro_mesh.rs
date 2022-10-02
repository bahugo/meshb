use derive_more::{Display, From};
use ndarray::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use num_traits::ToPrimitive;

use crate::patro_mesh_enums::{PatroMeshFormat, PatroCellType};
use crate::patro_cell::{PatroCell, Poi1Cell, Seg2Cell};
use crate::patro_node::PatroNode;

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

// #[derive(Debug, PartialEq, Clone)]
pub struct PatroMesh {
    // noeuds
    pub nodes: Vec<Rc<PatroNode>>,
    // mailles
    pub cells: Vec<Rc<dyn PatroCell>>,
    // gno [Group]: groupes de noeuds (dictionnaire de arrays de numéros de noeuds)
    pub gno: HashMap<String, Vec<Rc<PatroNode>>>,
    // gma [Group]: groupes de mailles (dictionnaire de arrays de numéros de mailles)
    pub gma: HashMap<String, Vec<Rc<dyn PatroCell>>>,
}

impl PatroMesh {
    pub fn new() -> Self {
        PatroMesh {
            nodes: vec![],
            cells: vec![],
            gno: HashMap::new(),
            gma: HashMap::new(),
        }
    }

    pub fn add_nodes(&mut self, nodes: &Array2<f64>, nodes_names: &[&str]) {
        for inode in 0..nodes.shape()[0]
        {
            let node_tmp = PatroNode {
                x: nodes[[inode, 0]],
                y: nodes[[inode, 1]],
                z: nodes[[inode, 2]],
                name: nodes_names[inode].to_string(),
            };
            self.nodes.push(Rc::new(node_tmp));
        };
    }

    pub fn add_cells(&mut self, connectivities: &[Array1<usize>], ty: PatroCellType) -> Result<Vec<usize>, &str> {
        match ty {
            PatroCellType::POI1 => {
                self.add_poi1_cells(connectivities)
            }
            PatroCellType::SEG2 => {
                self.add_seg2_cells(connectivities)
            }
            PatroCellType::TRIA3 => {
                unimplemented!()
            }
            PatroCellType::QUAD4 => {
                unimplemented!()
            }
            PatroCellType::PENTA6 => {
                unimplemented!()
            }
            PatroCellType::PYRAM5 => {
                unimplemented!()
            }
            PatroCellType::HEXA8 => {
                unimplemented!()
            }
        }
    }
    pub fn get_cell_name(cell_id: usize) -> String {
        format!("M{}", &(cell_id + 1))
    }
    pub fn add_poi1_cells(&mut self, connectivities: &[Array1<usize>]) -> Result<Vec<usize>, &str> {
        let mut cells = vec![];
        for nodes in connectivities.iter() {
            let cell_id = self.cells.len().to_usize().unwrap();
            let cell = Poi1Cell {
                co: [self.nodes[nodes[0]].clone(), ],
                name: Self::get_cell_name(cell_id).to_string(),
            };
            self.cells.push(Rc::new(cell));
            cells.push(cell_id);
        }
        Ok(cells)
    }
    pub fn add_seg2_cells(&mut self, connectivities: &[Array1<usize>]) -> Result<Vec<usize>, &str> {
        let mut cells = vec![];
        for nodes in connectivities.iter() {
            let cell_id = self.cells.len().to_usize().unwrap();
            let cell = Seg2Cell {
                co: [self.nodes[nodes[0]].clone(), self.nodes[nodes[1]].clone()],
                name: Self::get_cell_name(cell_id).to_string(),
            };
            self.cells.push(Rc::new(cell));
            cells.push(cell_id);
        }
        Ok(cells)
    }

    pub fn read_mesh(filename: &str, format: PatroMeshFormat) -> PatroMesh {
        let mut mesh = PatroMesh::new();
        match format {
            PatroMeshFormat::Mail => mesh.read_mail_format(filename),
        }
        mesh
    }
    pub fn read_mail_format(&self, filename: &str) {
        println!("Reading file {}", filename);

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        // TODO: parse contents with regex

        println!("Ended reading {}", filename);
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use ndarray::array;
    use crate::patro_mesh::PatroMesh;
    use crate::patro_mesh_enums::PatroCellType;
    use crate::patro_node::PatroNode;

    #[test]
    fn patro_mesh_init_empty_should_work() {
        let mesh = PatroMesh::new();
        assert_eq!(mesh.cells.len(), 0);
        assert_eq!(mesh.nodes.len(), 0);
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
    }

    #[test]
    fn patro_mesh_add_nodes_should_work() {
        let mut mesh = PatroMesh::new();
        let nodes_names = ["N12", "N2"];
        let nodes = array![[3., 0., 1.], [2., 1., 1.]];
        mesh.add_nodes(&nodes, &nodes_names);
        assert_eq!(mesh.nodes.len(), 2);
        assert_eq!(*mesh.nodes[0], PatroNode { x: 3.0, y: 0.0, z: 1.0, name: "N12".to_string() });
        assert_eq!(*mesh.nodes[1], PatroNode { x: 2.0, y: 1.0, z: 1.0, name: "N2".to_string() });
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
        let nodes_names = ["N21", "N222"];
        let nodes = array![[3.2, 0.3, 1.3], [2.1, 1.1, 1.1]];
        mesh.add_nodes(&nodes, &nodes_names);
        assert_eq!(mesh.nodes.len(), 4);
        assert_eq!(*mesh.nodes[2], PatroNode { x: 3.2, y: 0.3, z: 1.3, name: "N21".to_string() });
        assert_eq!(*mesh.nodes[3], PatroNode { x: 2.1, y: 1.1, z: 1.1, name: "N222".to_string() });
    }

    fn get_mesh_with_six_nodes() -> PatroMesh {
        let mut mesh = PatroMesh::new();
        let nodes_names = ["N1", "N2", "N3", "N4", "N5", "N6"];
        let nodes = array![[3., 0., 1.], [2., 1., 1.], [3.2, 0.3, 1.3], [2.1, 1.1, 1.1],
         [4.2, 0.3, 1.3], [3.1, 1.1, 1.1]];
        mesh.add_nodes(&nodes, &nodes_names);
        mesh
    }

    #[test]
    fn patro_mesh_add_cells_should_work_for_poi1() {
        let mut mesh = get_mesh_with_six_nodes();

        assert_eq!(mesh.cells.len(), 0);
        let new_cells = mesh.add_cells(
            &[array![0], array![2]],
            PatroCellType::POI1);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);
        let cell_co_1 = mesh.cells[0].get_co();
        assert_eq!(cell_co_1.len(), 1);
        assert_eq!(*cell_co_1[0], PatroNode { x: 3., y: 0., z: 1., name: "N1".to_string() });
        let cell_co_2 = mesh.cells[1].get_co();
        assert_eq!(cell_co_2.len(), 1);
        assert_eq!(*cell_co_2[0], PatroNode { x: 3.2, y: 0.3, z: 1.3, name: "N3".to_string() });
    }

    #[test]
    fn patro_mesh_add_cells_should_work_for_seg2()
    {
        let mut mesh = get_mesh_with_six_nodes();
        assert_eq!(mesh.cells.len(), 0);
        let new_cells = mesh.add_cells(
            &[array![0, 1], array![2, 3]],
            PatroCellType::SEG2);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);
        let cell_co_1 = mesh.cells[0].get_co();
        assert_eq!(*cell_co_1[0], PatroNode { x: 3., y: 0., z: 1., name: "N1".to_string() });
        assert_eq!(*cell_co_1[1], PatroNode { x: 2., y: 1., z: 1., name: "N2".to_string() });
        let cell_co_2 = mesh.cells[1].get_co();
        assert_eq!(*cell_co_2[0], PatroNode { x: 3.2, y: 0.3, z: 1.3, name: "N3".to_string() });
        assert_eq!(*cell_co_2[1], PatroNode { x: 2.1, y: 1.1, z: 1.1, name: "N4".to_string() });
    }
}
