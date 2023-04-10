use derive_more::{Display, From};
use ndarray::prelude::*;
use num_traits::ToPrimitive;
use std::cell::Cell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use crate::patro_cell::{PatroCell, Poi1Cell, Seg2Cell};
use crate::patro_mesh_enums::{PatroCellType, PatroMeshFormat};
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

// FIXME repasser à la version avec des Cell au lieu de refCell + faire une hashmap pour stocker
//  un id fixe pour chaque maille et chaque noeud et utiliser cet id partout pour référencer
//  les élts (par exemple dans les groupes, dans les connectivités, etc...)

// #[derive(Debug, PartialEq, Clone)]
pub struct PatroMesh {
    pub next_node_id: usize,
    pub next_cell_id: usize,
    // noeuds
    pub nodes: HashMap<usize, Rc<Cell<PatroNode>>>,
    // mailles
    pub cells: HashMap<usize, Rc<dyn PatroCell>>,
    // gno [Group]: groupes de noeuds (dictionnaire de arrays de numéros de noeuds)
    pub gno: HashMap<String, Vec<usize>>,
    // gma [Group]: groupes de mailles (dictionnaire de arrays de numéros de mailles)
    pub gma: HashMap<String, Vec<usize>>,
}

impl PatroMesh {
    pub fn new() -> Self {
        PatroMesh {
            next_node_id: 0,
            next_cell_id: 0,
            nodes: HashMap::new(),
            cells: HashMap::new(),
            gno: HashMap::new(),
            gma: HashMap::new(),
        }
    }

    pub fn add_nodes(&mut self, nodes: &Array2<f64>, nodes_names: &[&'static str]) {
        for inode in 0..nodes.shape()[0] {
            let node_tmp = Cell::new(PatroNode {
                x: nodes[[inode, 0]],
                y: nodes[[inode, 1]],
                z: nodes[[inode, 2]],
                name: nodes_names[inode],
            });
            self.nodes.insert(self.next_node_id, Rc::new(node_tmp));
            // incrément du prochain node_id
            self.next_node_id += 1;
        }
    }

    pub fn edit_node(
        &mut self,
        index: &usize,
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
    ) -> bool {
        match self.nodes.get(index) {
            Some(node) => {
                let mut node_tmp = node.clone().get();

                if let Some(val) = x {
                    node_tmp.x = val;
                }
                if let Some(val) = y {
                    node_tmp.y = val;
                }
                if let Some(val) = z {
                    node_tmp.z = val;
                }
                self.nodes[index].set(node_tmp);
                return true;
            }
            None => return false,
        }
    }

    pub fn add_cells(
        &mut self,
        connectivities: &[Array1<usize>],
        ty: PatroCellType,
    ) -> Result<Vec<usize>, &str> {
        match ty {
            PatroCellType::POI1 => self.add_cells_of_type::<Poi1Cell>(connectivities),
            PatroCellType::SEG2 => self.add_cells_of_type::<Seg2Cell>(connectivities),
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

    pub fn get_cell_co(&mut self, cell_id: &usize) -> Result<Vec<Rc<Cell<PatroNode>>>, &'static str> {
        let cell_opt = self.cells.get(cell_id);

        if Option::is_none(&cell_opt) {
            return Err("cell_id not found in cells");
        }

        let node_ids = cell_opt.unwrap().clone().get_co();
        let out = node_ids
            .iter()
            .map(|node_id| self.nodes[&node_id].clone())
            .collect();

        Ok(out)
    }

    pub fn create_one_cell<T>(&mut self, connectivity: &Array1<usize>) -> Result<T, &str>
    where
        T: PatroCell,
    {
        let cell = T::new(connectivity);
        Ok(cell)
    }

    pub fn add_cells_of_type<T>(
        &mut self,
        connectivities: &[Array1<usize>],
    ) -> Result<Vec<usize>, &str>
    where
        T: PatroCell + 'static,
    {
        let mut cells = vec![];
        for nodes in connectivities.iter() {
            let cell = self.create_one_cell::<T>(nodes).unwrap();
            self.cells.insert(self.next_cell_id, Rc::new(cell));
            cells.push(self.next_cell_id);
            self.next_cell_id += 1;
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
    use crate::patro_mesh::PatroMesh;
    use crate::patro_mesh_enums::PatroCellType;
    use crate::patro_node::PatroNode;
    use ndarray::array;

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
        let node = mesh.nodes[&0].get();
        assert_eq!(
            (mesh.nodes[&0].get()),
            PatroNode {
                x: 3.0,
                y: 0.0,
                z: 1.0,
                name: "N12"
            }
        );
        assert_eq!(
            (mesh.nodes[&1].get()),
            PatroNode {
                x: 2.0,
                y: 1.0,
                z: 1.0,
                name: "N2"
            }
        );
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
        let nodes_names = ["N21", "N222"];
        let nodes = array![[3.2, 0.3, 1.3], [2.1, 1.1, 1.1]];
        mesh.add_nodes(&nodes, &nodes_names);
        assert_eq!(mesh.nodes.len(), 4);
        assert_eq!(
            (mesh.nodes[&2].get()),
            PatroNode {
                x: 3.2,
                y: 0.3,
                z: 1.3,
                name: "N21"
            }
        );
        assert_eq!(
            (mesh.nodes[&3].get()),
            PatroNode {
                x: 2.1,
                y: 1.1,
                z: 1.1,
                name: "N222"
            }
        );
    }

    fn get_mesh_with_six_nodes() -> PatroMesh {
        let mut mesh = PatroMesh::new();
        let nodes_names = ["N1", "N2", "N3", "N4", "N5", "N6"];
        let nodes = array![
            [3., 0., 1.],
            [2., 1., 1.],
            [3.2, 0.3, 1.3],
            [2.1, 1.1, 1.1],
            [4.2, 0.3, 1.3],
            [3.1, 1.1, 1.1]
        ];
        mesh.add_nodes(&nodes, &nodes_names);
        mesh
    }

    #[test]
    fn patro_mesh_add_cells_should_work_for_poi1() {
        let mut mesh = get_mesh_with_six_nodes();

        assert_eq!(mesh.cells.len(), 0);
        let new_cells = mesh.add_cells(&[array![0], array![2]], PatroCellType::POI1);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);
        let cell_co_1 = mesh.get_cell_co(&0).unwrap();
        assert_eq!(cell_co_1.len(), 1);
        assert_eq!(
            (cell_co_1[0].clone().get()),
            PatroNode {
                x: 3.,
                y: 0.,
                z: 1.,
                name: "N1"
            }
        );
        let cell_co_2 = mesh.get_cell_co(&1).unwrap();
        assert_eq!(cell_co_2.len(), 1);
        assert_eq!(
            (cell_co_2[0].clone().get()),
            PatroNode {
                x: 3.2,
                y: 0.3,
                z: 1.3,
                name: "N3"
            }
        );
    }

    #[test]
    fn patro_mesh_add_cells_should_work_for_seg2() {
        let mut mesh = get_mesh_with_six_nodes();
        assert_eq!(mesh.cells.len(), 0);
        let new_cells = add_two_seg2_cells(&mut mesh);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);
        let cell_co_1 = mesh.get_cell_co(&0).unwrap();
        assert_eq!(
            (cell_co_1[0].clone().get()),
            PatroNode {
                x: 3.,
                y: 0.,
                z: 1.,
                name: "N1"
            }
        );
        assert_eq!(
            (cell_co_1[1].clone().get()),
            PatroNode {
                x: 2.,
                y: 1.,
                z: 1.,
                name: "N2"
            }
        );
        let cell_co_2 = mesh.get_cell_co(&1).unwrap();
        assert_eq!(
            (cell_co_2[0].clone().get()),
            PatroNode {
                x: 3.2,
                y: 0.3,
                z: 1.3,
                name: "N3"
            }
        );
        assert_eq!(
            (cell_co_2[1].clone().get()),
            PatroNode {
                x: 2.1,
                y: 1.1,
                z: 1.1,
                name: "N4"
            }
        );
    }

    fn add_two_seg2_cells(mesh: &mut PatroMesh) -> Result<Vec<usize>, &str> {
        let new_cells = mesh.add_cells(&[array![0, 1], array![2, 3]], PatroCellType::SEG2);
        new_cells
    }

    #[test]
    fn should_be_able_to_change_a_node_already_used() {
        let mut mesh = get_mesh_with_six_nodes();
        {
            let _new_cells = add_two_seg2_cells(&mut mesh);
            {
                let result = mesh.edit_node(&0, Some(10.2_f64), Some(0.2_f64), None);
                assert_eq!(result, true);
            }
            let first_node = mesh.nodes[&0].clone().get();

            assert_eq!(first_node.x, 10.2_f64);
            assert_eq!(first_node.y, 0.2_f64);
            assert_eq!(first_node.z, 1.0_f64);
        }
    }
}
