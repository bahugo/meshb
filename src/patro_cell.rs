use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::rc::Rc;
use ndarray::Array1;
use patro_node::PatroNode;
use crate::patro_node;


pub trait PatroCell {
    fn get_name(&self) -> &str;
    fn get_co(&self) -> Vec<Rc<RefCell<PatroNode>>>;
    fn new(connectivity: Array1<Rc<RefCell<PatroNode>>>, name: String) -> Self where Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Poi1Cell {
    pub co: [Rc<RefCell<PatroNode>>; 1],
    pub name: Cow<'static, str>,
}

impl PatroCell for Poi1Cell {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_co(&self) -> Vec<Rc<RefCell<PatroNode>>> {
        self.co.to_vec()
    }

    fn new(connectivity: Array1<Rc<RefCell<PatroNode>>>, name: String) -> Poi1Cell {
        Poi1Cell { co: [connectivity[0].clone()], name: Cow::Owned(name) }
    }
}

#[derive(Debug, Clone)]
pub struct Seg2Cell {
    pub co: [Rc<RefCell<PatroNode>>; 2],
    pub name: Cow<'static, str>,
}

impl PatroCell for Seg2Cell {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_co(&self) -> Vec<Rc<RefCell<PatroNode>>> {
        self.co.to_vec()
    }
    fn new(connectivity: Array1<Rc<RefCell<PatroNode>>>, name: String) -> Seg2Cell {
        Seg2Cell { co: [connectivity[0].clone(), connectivity[1].clone()], name: Cow::Owned(name) }
    }
}

#[derive(Debug, Clone)]
pub struct Tria3Cell {
    pub co: [Rc<PatroNode>; 3],
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct Quad4Cell {
    pub co: [Rc<PatroNode>; 4],
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct Penta6Cell {
    pub co: [Rc<PatroNode>; 6],
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct Pyram5Cell {
    pub co: [Rc<PatroNode>; 5],
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct Hexa8Cell {
    pub co: [Rc<PatroNode>; 8],
    pub name: Cow<'static, str>,
}
