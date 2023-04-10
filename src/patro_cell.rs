use std::rc::Rc;
use ndarray::Array1;
use patro_node::PatroNode;
use crate::patro_node;


pub trait PatroCell {
    fn get_co(&self) -> Vec<usize>;
    fn new(connectivity: &Array1<usize>) -> Self where Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Poi1Cell {
    pub co: [usize; 1],
}

impl PatroCell for Poi1Cell {

    fn get_co(&self) -> Vec<usize> {
        self.co.to_vec()
    }

    fn new(connectivity: &Array1<usize>) -> Poi1Cell {
        Poi1Cell { co: [connectivity[0].clone()]}
    }
}

#[derive(Debug, Clone)]
pub struct Seg2Cell {
    pub co: [usize; 2],
}

impl PatroCell for Seg2Cell {

    fn get_co(&self) -> Vec<usize> {
        self.co.to_vec()
    }
    fn new(connectivity: &Array1<usize>) -> Seg2Cell {
        Seg2Cell { co: [connectivity[0].clone(), connectivity[1].clone()]}
    }
}

#[derive(Debug, Clone)]
pub struct Tria3Cell {
    pub co: [Rc<PatroNode>; 3],
}

#[derive(Debug, Clone)]
pub struct Quad4Cell {
    pub co: [Rc<PatroNode>; 4],
}

#[derive(Debug, Clone)]
pub struct Penta6Cell {
    pub co: [Rc<PatroNode>; 6],
}

#[derive(Debug, Clone)]
pub struct Pyram5Cell {
    pub co: [Rc<PatroNode>; 5],
}

#[derive(Debug, Clone)]
pub struct Hexa8Cell {
    pub co: [Rc<PatroNode>; 8],
}
