use derive_more::{Display};

#[derive(Display)]
pub enum MeshFormat {
    Mail,
}

#[derive(Display)]
pub enum CellType {
    POI1,
    SEG2,
    TRIA3,
    QUAD4,
    PENTA6,
    PYRAM5,
    HEXA8,
}
