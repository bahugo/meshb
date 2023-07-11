use ndarray::Array1;


pub trait PatroCell{
    fn get_co(&self) -> Array1<usize>;
    fn new(connectivity: &Array1<usize>) -> Result<Self, &'static str> where Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Poi1Cell {
    pub co: Array1<usize>,
}

impl PatroCell for Poi1Cell {
    fn get_co(&self) -> Array1<usize> {
        self.co.clone()
    }

    fn new(connectivity: &Array1<usize>) -> Result<Poi1Cell, &'static str>{

        if connectivity.len() != 1 {
            return Err("Poi1Cell connectivity must be of length 1");
        }
        Ok(Poi1Cell {
            co: connectivity.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Seg2Cell {
    pub co: Array1<usize>,
}

impl PatroCell for Seg2Cell {
    fn get_co(&self) -> Array1<usize> {
        self.co.clone()
    }
    fn new(connectivity: &Array1<usize>) -> Result<Seg2Cell, &'static str> {
        if connectivity.len() != 2 {
            return Err("Poi1Cell connectivity must be of length 2");
        }
        Ok(Seg2Cell {
            co: connectivity.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Tria3Cell {
    pub co: Array1<usize>,
}

#[derive(Debug, Clone)]
pub struct Quad4Cell {
    pub co: Array1<usize>,
}

#[derive(Debug, Clone)]
pub struct Penta6Cell {
    pub co: Array1<usize>,
}

#[derive(Debug, Clone)]
pub struct Pyram5Cell {
    pub co: Array1<usize>,
}

#[derive(Debug, Clone)]
pub struct Hexa8Cell {
    pub co: Array1<usize>,
}
