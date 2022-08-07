use derive_more::{Display, From};
#[derive(Debug, PartialEq, Clone, Display)]
pub enum PatroMeshFormat {
    Mail,
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum PatroCellType {
    POI1,
    SEG2,
    TRIA3,
    QUAD4,
    PENTA6,
    PYRAM5,
    HEXA8,
}
