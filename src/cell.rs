use ndarray::Array1;

use crate::lib::CellType;


#[derive(Debug, Clone)]
pub struct MeshCell {
    pub ty: CellType,
    pub co: Array1<usize>,

}

impl MeshCell {
    pub fn get_co(&self) -> Array1<usize> {
        self.co.clone()
    }

    pub fn new(cell_type: CellType, connectivity: &Array1<usize>) -> Result<MeshCell, &'static str>{

        if connectivity.len() != cell_type.get_nb_of_connectivities() {
            return Err("Poi1Cell connectivity must be of length 1");
        }
        Ok(MeshCell {
            ty: cell_type,
            co: connectivity.clone(),
        })
    }
}
