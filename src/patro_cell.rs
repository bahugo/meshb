use ndarray::Array1;

pub trait PatroCell {
    fn get_co(&self) -> Array1<usize>;
    // TODO : RENVOYER UN RESULT POUR GERER LE CAS OU ON NE DONNE PAS LE BON NOMBRE DE CONNECTIVITE
    fn new(connectivity: &Array1<usize>) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Poi1Cell {
    pub co: Array1<usize>,
}

impl PatroCell for Poi1Cell {
    fn get_co(&self) -> Array1<usize> {
        self.co.clone()
    }

    fn new(connectivity: &Array1<usize>) -> Poi1Cell {
        Poi1Cell {
            co: connectivity.clone(),
        }
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
    fn new(connectivity: &Array1<usize>) -> Seg2Cell {
        Seg2Cell {
            co: connectivity.clone(),
        }
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
