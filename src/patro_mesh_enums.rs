use derive_more::{Display};

#[derive(Display)]
pub enum PatroMeshFormat {
    Mail,
}

#[derive(Display)]
pub enum PatroCellType {
    POI1,
    SEG2,
    TRIA3,
    QUAD4,
    PENTA6,
    PYRAM5,
    HEXA8,
}
