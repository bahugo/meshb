use derive_more::Display;

#[derive(Display)]
pub enum MeshFormat {
    Mail,
}

// / POI1 points )
// / SEG2 / SEG3 / SEG4 segments )
// / TRIA3 / TRIA6 / TRIA7 triangles )
// / QUAD4 / QUAD8 / QUAD9 quadrangles ) connectivité
// / HEXA8 / HEXA20 / HEXA27 hexaèdres ) des mailles
// / PENTA6 / PENTA15 / PENTA18 pentaèdres )
// / TETRA4 / TETRA10 tétraèdres )
// / PYRAM5 / PYRAM13 pyramides )
#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    POI1,
    SEG2,
    SEG3,
    SEG4,
    TRIA3,
    TRIA6,
    TRIA7,
    QUAD4,
    QUAD8,
    QUAD9,
    HEXA8,
    HEXA20,
    HEXA27,
    PENTA6,
    PENTA15,
    PENTA18,
    TETRA4,
    TETRA10,
    PYRAM5,
    PYRAM13,
}
impl CellType {
    pub fn from_string(value: &str) -> Result<Self, &str> {
        return match value {
            "POI1" => Ok(CellType::POI1),
            "SEG2" => Ok(CellType::SEG2),
            "SEG3" => Ok(CellType::SEG3),
            "SEG4" => Ok(CellType::SEG4),
            "TRIA3" => Ok(CellType::TRIA3),
            "TRIA6" => Ok(CellType::TRIA6),
            "TRIA7" => Ok(CellType::TRIA7),
            "QUAD4" => Ok(CellType::QUAD4),
            "QUAD8" => Ok(CellType::QUAD8),
            "QUAD9" => Ok(CellType::QUAD9),
            "HEXA8" => Ok(CellType::HEXA8),
            "HEXA20" => Ok(CellType::HEXA20),
            "HEXA27" => Ok(CellType::HEXA27),
            "PENTA6" => Ok(CellType::PENTA6),
            "PENTA15" => Ok(CellType::PENTA15),
            "PENTA18" => Ok(CellType::PENTA18),
            "TETRA4" => Ok(CellType::TETRA4),
            "TETRA10" => Ok(CellType::TETRA10),
            "PYRAM5" => Ok(CellType::PYRAM5),
            "PYRAM13" => Ok(CellType::PYRAM13),
            _ => Err("cell_type not implemented"),
        };
    }
    pub fn get_nb_of_connectivities(&self) -> usize {
        return match &self {
            CellType::POI1 => 1,
            CellType::SEG2 => 2,
            CellType::SEG3 => 3,
            CellType::SEG4 => 4,
            CellType::TRIA3 => 3,
            CellType::TRIA6 => 6,
            CellType::TRIA7 => 7,
            CellType::QUAD4 => 4,
            CellType::QUAD8 => 8,
            CellType::QUAD9 => 9,
            CellType::HEXA8 => 8,
            CellType::HEXA20 => 20,
            CellType::HEXA27 => 27,
            CellType::PENTA6 => 6,
            CellType::PENTA15 => 15,
            CellType::PENTA18 => 18,
            CellType::TETRA4 => 4,
            CellType::TETRA10 => 10,
            CellType::PYRAM5 => 5,
            CellType::PYRAM13 => 13,
        };
    }
}

#[cfg(test)]
mod tests {

    use crate::mesh_enums::CellType;

    #[test]
    fn cell_type_connectivity_nb_should_work() {
        assert_eq!(CellType::POI1.get_nb_of_connectivities(), 1);
        assert_eq!(CellType::SEG2.get_nb_of_connectivities(), 2);
        assert_eq!(CellType::SEG3.get_nb_of_connectivities(), 3);
        assert_eq!(CellType::SEG4.get_nb_of_connectivities(), 4);
        assert_eq!(CellType::TRIA3.get_nb_of_connectivities(), 3);
        assert_eq!(CellType::TRIA6.get_nb_of_connectivities(), 6);
        assert_eq!(CellType::TRIA7.get_nb_of_connectivities(), 7);
        assert_eq!(CellType::QUAD4.get_nb_of_connectivities(), 4);
        assert_eq!(CellType::QUAD8.get_nb_of_connectivities(), 8);
        assert_eq!(CellType::QUAD9.get_nb_of_connectivities(), 9);
        assert_eq!(CellType::HEXA8.get_nb_of_connectivities(), 8);
        assert_eq!(CellType::HEXA20.get_nb_of_connectivities(), 20);
        assert_eq!(CellType::HEXA27.get_nb_of_connectivities(), 27);
        assert_eq!(CellType::PENTA6.get_nb_of_connectivities(), 6);
        assert_eq!(CellType::PENTA15.get_nb_of_connectivities(), 15);
        assert_eq!(CellType::PENTA18.get_nb_of_connectivities(), 18);
        assert_eq!(CellType::TETRA4.get_nb_of_connectivities(), 4);
        assert_eq!(CellType::TETRA10.get_nb_of_connectivities(), 10);
        assert_eq!(CellType::PYRAM5.get_nb_of_connectivities(), 5);
        assert_eq!(CellType::PYRAM13.get_nb_of_connectivities(), 13);
    }
}
