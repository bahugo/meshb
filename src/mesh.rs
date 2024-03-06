use core::fmt;

use std::collections::{HashMap, HashSet};
use std::fs;

use crate::cell::MeshCell;
use crate::lib::mail_parser::mail_parser;
use crate::mesh_enums::{CellType, MeshFormat};
use crate::node::Node;
use crate::parsers::tokens::{MailParseOutput, NodeProp};

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

pub struct MeshError {
    message: String,
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MeshError {}", self.message.clone())
    }
}
impl fmt::Debug for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MeshError {}", self.message.clone())
    }
}

// #[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    pub next_node_id: usize,
    pub next_cell_id: usize,
    // noeuds
    pub nodes: HashMap<usize, Node>,
    // mailles
    pub cells: HashMap<usize, MeshCell>,
    // gno [Group]: groupes de noeuds (dictionnaire de arrays de numéros de noeuds)
    pub gno: HashMap<&'static str, Vec<usize>>,
    // gma [Group]: groupes de mailles (dictionnaire de arrays de numéros de mailles)
    pub gma: HashMap<&'static str, Vec<usize>>,
    // mapping from node names to node ids
    pub nodes_name_to_id: HashMap<Box<str>, usize>,
}

impl<'a> Mesh {
    pub fn new() -> Self {
        Mesh {
            next_node_id: 0,
            next_cell_id: 0,
            nodes: HashMap::new(),
            cells: HashMap::new(),
            gno: HashMap::new(),
            gma: HashMap::new(),
            nodes_name_to_id: HashMap::new(),
        }
    }

    pub fn add_nodes(&mut self, nodes: Vec<NodeProp<'a>>) {
        for inode in 0..nodes.len() {
            let node_prop = &nodes[inode];
            let node_tmp = Node {
                x: node_prop.x.into(),
                y: node_prop.y.into(),
                z: node_prop.z.into(),
            };
            self.nodes.insert(self.next_node_id, node_tmp);
            if let Some(node_name) = node_prop.name {
                self.nodes_name_to_id.insert(node_name.into(), self.next_node_id);
            }            // incrément du prochain node_id
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
        if let Some(node) = self.nodes.get_mut(index) {
            // let &mut node_tmp = node;

            if let Some(val) = x {
                node.x = val;
            }
            if let Some(val) = y {
                node.y = val;
            }
            if let Some(val) = z {
                node.z = val;
            }
            true
        } else {
            false
        }
    }

    pub fn add_cells(
        &mut self,
        connectivities: &[Vec<usize>],
        ty: CellType,
    ) -> Result<Vec<usize>, &str> {
        self.add_cells_of_type(ty, connectivities)
    }

    pub fn get_cell_name(cell_id: usize) -> String {
        format!("M{}", &(cell_id + 1))
    }

    pub fn get_cell_co(&self, cell_id: usize) -> Result<Vec<usize>, MeshError> {
        let node_ids = match self.cells.get(&cell_id) {
            Some(val) => val.get_co(),
            None => {
                let mess = format!("cell_id {} not found in cells", cell_id);
                return Err(MeshError { message: mess });
            }
        };

        Ok(node_ids)
    }

    pub fn create_one_cell(
        cell_type: CellType,
        connectivity: &Vec<usize>,
    ) -> Result<MeshCell, &'static str> {
        let cell = MeshCell::new(cell_type, connectivity);
        cell
    }

    pub fn add_a_cell(
        &mut self,
        cell_type: CellType,
        connectivity: &Vec<usize>) -> Result<usize, &'static str> {

        let cell = match Self::create_one_cell(cell_type.clone(), connectivity) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        self.cells.insert(self.next_cell_id, cell);
        let cell_id = self.next_cell_id;
        self.next_cell_id += 1;
        Ok(cell_id)
    }

    pub fn add_cells_of_type(
        &mut self,
        cell_type: CellType,
        connectivities: &[Vec<usize>],
    ) -> Result<Vec<usize>, &'static str> {
        let mut cells = vec![];
        for nodes in connectivities.iter() {
            let cell_id = self.add_a_cell(cell_type.clone(), nodes)?;
            cells.push(cell_id);
        }
        Ok(cells)
    }

    fn extract_cell_result(cell: Result<MeshCell, &'static str>) -> Result<MeshCell, &'static str> {
        match cell {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        }
    }

    pub fn edit_cell(&mut self, index: usize, connectivity: &Vec<usize>, ty: CellType) -> bool {
        let val: Result<MeshCell, &'static str>;
        val = Self::extract_cell_result(MeshCell::new(ty, connectivity));
        match val {
            Ok(ok_val) => {
                if self.cells.insert(index, ok_val).is_none() {
                    return false;
                };
                true
            }
            Err(_e) => false,
        }
    }
    pub fn create_node_group(
        &mut self,
        name: &'static str,
        node_ids: &Vec<usize>,
    ) -> Result<(), &'static str> {
        let unique_node_ids: HashSet<usize> = node_ids.clone().into_iter().collect();
        let existing_node_ids: HashSet<usize> = self.nodes.keys().cloned().collect();
        let intersection = existing_node_ids
            .intersection(&unique_node_ids)
            .collect::<Vec<&usize>>();

        if intersection.len() != unique_node_ids.len() {
            return Err("At least one node_id is not contained in mesh");
        }
        let mut target_node_ids = unique_node_ids.into_iter().collect::<Vec<usize>>();
        target_node_ids.sort();
        self.gma.insert(name, target_node_ids);
        Ok(())
    }
    pub fn create_cell_group(
        &mut self,
        name: &'static str,
        cell_ids: &Vec<usize>,
    ) -> Result<(), &'static str> {
        let unique_cell_ids: HashSet<usize> = cell_ids.clone().into_iter().collect();
        let existing_cell_ids: HashSet<usize> = self.cells.keys().cloned().collect();
        let intersection = existing_cell_ids
            .intersection(&unique_cell_ids)
            .collect::<Vec<&usize>>();

        if intersection.len() != unique_cell_ids.len() {
            return Err("At least one cell_id is not contained in mesh");
        }
        let mut target_cell_ids = unique_cell_ids.into_iter().collect::<Vec<usize>>();
        target_cell_ids.sort();
        self.gma.insert(name, target_cell_ids);
        Ok(())
    }

    pub fn create_from_parser_output(parser_output: MailParseOutput) -> Mesh {
        let mut mesh = Mesh::new();

        mesh.add_nodes(parser_output.nodes.to_owned());

        for cell in parser_output.cells {
            let connectivities: Vec<usize> = cell.nodes.into_iter()
                .map(|x| {
                    let node_name = String::from(x).into_boxed_str();
                    let node_id = mesh.nodes_name_to_id.get(&node_name).unwrap().clone();
                    node_id
                })
                .collect();
            let _ = mesh.add_a_cell(cell.cell_type, &connectivities);
        }
        for _group in parser_output.groups {
            todo!()
        }
        mesh
    }

    pub fn read_mesh(filename: &str, format: MeshFormat) -> Self {
        println!("Reading file {}", filename);

        let content = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let parser_output = match format {
            MeshFormat::Mail => Mesh::read_mail_format(&content),
        };
        let output = match parser_output {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };

        let mesh = Mesh::create_from_parser_output(output);
        mesh
    }

    pub fn read_mail_format(content: &'a str) -> Result<MailParseOutput<'a>, &'a str> {
        let output = mail_parser(content);
        match output {
            Ok(val) => Ok(val),
            _ => Err("parser failed"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::mesh::Mesh;
    use crate::mesh_enums::CellType;
    use crate::node::Node;
    use crate::parsers::tokens::NodeProp;

    #[test]
    fn mesh_init_empty_should_work() {
        let mesh = Mesh::new();
        assert_eq!(mesh.cells.len(), 0);
        assert_eq!(mesh.nodes.len(), 0);
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
    }

    #[test]
    fn mesh_add_nodes_should_work() {
        let mut mesh = Mesh::new();
        let nodes = vec![
            NodeProp {
                x: 3.,
                y: 0.,
                z: 1.,
                name: Some("N12"),
            },
            NodeProp {
                x: 2.,
                y: 1.,
                z: 1.,
                name: Some("N2"),
            },
        ];
        mesh.add_nodes(nodes);
        assert_eq!(mesh.nodes.len(), 2);
        let _node = mesh.nodes.get(&0);
        assert_eq!(
            (*mesh.nodes.get(&0).unwrap()),
            Node {
                x: 3.0,
                y: 0.0,
                z: 1.0,
            }
        );
        assert_eq!(
            (*mesh.nodes.get(&1).unwrap()),
            Node {
                x: 2.0,
                y: 1.0,
                z: 1.0,
            }
        );
        assert_eq!(mesh.gno.len(), 0);
        assert_eq!(mesh.gma.len(), 0);
        let nodes = vec![
            NodeProp {
                x: 3.2,
                y: 0.3,
                z: 1.3,
                name: Some("N21"),
            },
            NodeProp {
                x: 2.1,
                y: 1.1,
                z: 1.1,
                name: Some("N222"),
            },
        ];
        mesh.add_nodes(nodes);
        assert_eq!(mesh.nodes.len(), 4);
        assert_eq!(
            (*mesh.nodes.get(&2).unwrap()),
            Node {
                x: 3.2,
                y: 0.3,
                z: 1.3,
            }
        );
        assert_eq!(
            (*mesh.nodes.get(&3).unwrap()),
            Node {
                x: 2.1,
                y: 1.1,
                z: 1.1,
            }
        );
    }

    fn get_mesh_with_six_nodes() -> Mesh {
        let mut mesh = Mesh::new();
        let nodes = vec![
            NodeProp {
                x: 3.,
                y: 0.,
                z: 1.,
                name: Some("N1"),
            },
            NodeProp {
                x: 2.,
                y: 1.,
                z: 1.,
                name: Some("N2"),
            },
            NodeProp {
                x: 3.2,
                y: 0.3,
                z: 1.3,
                name: Some("N3"),
            },
            NodeProp {
                x: 2.1,
                y: 1.1,
                z: 1.1,
                name: Some("N4"),
            },
            NodeProp {
                x: 4.2,
                y: 0.3,
                z: 1.3,
                name: Some("N5"),
            },
            NodeProp {
                x: 3.1,
                y: 1.1,
                z: 1.1,
                name: Some("N6"),
            },
        ];
        mesh.add_nodes(nodes);
        mesh
    }

    #[test]
    fn mesh_add_cells_should_return_err_when_connectivity_has_bad_len() {
        let mut mesh = get_mesh_with_six_nodes();
        let new_cells = mesh.add_cells(&[vec![0], vec![2, 2]], CellType::POI1);
        assert_eq!(new_cells.is_err(), true);
    }

    #[test]
    fn mesh_add_cells_should_work_for_poi1() {
        let mut mesh = get_mesh_with_six_nodes();

        assert_eq!(mesh.cells.len(), 0);
        let new_cells = mesh.add_cells(&[vec![0], vec![2]], CellType::POI1);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);

        let cell_co_not_found = mesh.get_cell_co(1112);
        assert!(matches!(cell_co_not_found, Err(_mesh_error)));

        let cell_co_1 = mesh.get_cell_co(0).unwrap();
        assert_eq!(cell_co_1.len(), 1);
        assert_eq!( cell_co_1[0], 0);
        let cell_co_2 = mesh.get_cell_co(1).unwrap();
        assert_eq!(cell_co_2.len(), 1);
        assert_eq!(cell_co_2[0], 2);
    }

    #[test]
    fn mesh_add_cells_should_work_for_seg2() {
        let mut mesh = get_mesh_with_six_nodes();
        assert_eq!(mesh.cells.len(), 0);
        let new_cells = add_two_seg2_cells(&mut mesh);
        assert_eq!(new_cells.unwrap(), vec![0, 1]);
        assert_eq!(mesh.cells.len(), 2);
        let cell_co_1 = mesh.get_cell_co(0).unwrap();
        assert_eq!( cell_co_1[0], 0);
        assert_eq!( cell_co_1[1], 1);
        let cell_co_2 = mesh.get_cell_co(1).unwrap();
        let _cell_co_3 = mesh.get_cell_co(1).unwrap();
        assert_eq!( cell_co_2[0], 2);
        assert_eq!( cell_co_2[1], 3);
    }

    fn add_two_seg2_cells<'a>(mesh: &'a mut Mesh) -> Result<Vec<usize>, &'a str> {
        let new_cells = mesh.add_cells(&[vec![0, 1], vec![2, 3]], CellType::SEG2);
        new_cells
    }

    #[test]
    fn should_be_able_to_change_a_node_already_used() {
        let mut mesh = get_mesh_with_six_nodes();
        {
            let _new_cells = add_two_seg2_cells(&mut mesh);
            let result = mesh.edit_node(&0, Some(10.2_f64), Some(0.2_f64), None);
            assert_eq!(result, true);
            let first_node = &mesh.nodes[&0];

            assert_eq!(first_node.x, 10.2_f64);
            assert_eq!(first_node.y, 0.2_f64);
            assert_eq!(first_node.z, 1.0_f64);
        }
    }

    #[test]
    fn should_be_able_to_edit_a_cell() {
        let mut mesh = get_mesh_with_six_nodes();
        {
            let _new_cells = add_two_seg2_cells(&mut mesh);
            let first_cell = &mesh.cells[&0];
            assert_eq!(first_cell.get_co().len(), 2);
            mesh.edit_cell(0, &vec![0], CellType::POI1);
            let first_cell = &mesh.cells[&0];
            assert_eq!(first_cell.get_co().len(), 1);
        }
    }
    #[test]
    fn create_node_group_should_work() {
        let mut mesh = get_mesh_with_six_nodes();
        let _new_cells = add_two_seg2_cells(&mut mesh).unwrap();
        let group_node_ids = vec![0, 2, 4];
        assert_eq!(mesh.create_node_group("GROUP1", &group_node_ids), Ok(()));
        assert_eq!(
            mesh.create_node_group("GROUP_NOT_POSSIBLE", &vec![1000])
                .is_err(),
            true
        );
        let gma = &mesh.gma.clone();
        let actual_node_ids = gma.get("GROUP1").unwrap();
        assert_eq!(actual_node_ids, &group_node_ids.clone());
    }
    #[test]
    fn create_cell_group_should_work() {
        let mut mesh = get_mesh_with_six_nodes();
        let new_cells = add_two_seg2_cells(&mut mesh).unwrap();
        let group_cell_ids = new_cells.clone();
        assert_eq!(mesh.create_cell_group("GROUP1", &group_cell_ids), Ok(()));
        assert_eq!(
            mesh.create_cell_group("GROUP_NOT_POSSIBLE", &vec![1000])
                .is_err(),
            true
        );
        let gma = &mesh.gma.clone();
        let actual_cell_ids = gma.get("GROUP1").unwrap();
        assert_eq!(actual_cell_ids, &new_cells.clone());
    }
}
