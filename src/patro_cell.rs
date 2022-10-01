use std::rc::Rc;
use patro_node::PatroNode;
use crate::patro_node;


pub trait PatroCell {
    fn get_name(&self) -> &str;
    fn get_co(&self) -> Vec<Rc<PatroNode>>;
}

#[derive(Debug, Clone)]
pub struct Poi1Cell{
    pub co: [Rc<PatroNode>; 1],
    pub name: String
}

impl PatroCell for Poi1Cell {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_co(&self) -> Vec<Rc<PatroNode>> {
        self.co.to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct Seg2Cell{
    pub co: [Rc<PatroNode>; 2],
    pub name: String
}

impl PatroCell for Seg2Cell {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_co(&self) -> Vec<Rc<PatroNode>> {
        self.co.to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct Tria3Cell{
    pub co: [Rc<PatroNode>; 3],
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Quad4Cell{
    pub co: [Rc<PatroNode>; 4],
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Penta6Cell{
    pub co: [Rc<PatroNode>; 6],
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Pyram5Cell{
    pub co: [Rc<PatroNode>; 5],
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Hexa8Cell{
    pub co: [Rc<PatroNode>; 8],
    pub name: String
}
