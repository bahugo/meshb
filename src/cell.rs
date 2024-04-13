use crate::lib::CellType;


#[derive(Debug, PartialEq, Clone)]
pub struct MeshCell {
    pub ty: CellType,
    pub co: Vec<usize>,

}

impl MeshCell {
    pub fn get_co(&self) -> Vec<usize> {
        self.co.clone()
    }

    pub fn new(cell_type: CellType, connectivity: &Vec<usize>) -> Result<MeshCell, &'static str>{

        if connectivity.len() != cell_type.get_nb_of_connectivities() {
            return Err("connectivity not implemented for this cell_type");
        }
        Ok(MeshCell {
            ty: cell_type,
            co: connectivity.clone(),
        })
    }
}
