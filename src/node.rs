use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use num_traits::abs;


#[derive(Debug, Clone)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: Box<str>,
}
impl Display for Node{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} ({}, {}, {})", self.name, self.x, self.y, self.z, )
    }
}
impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool {
        abs(self.x - other.x) <= 1e-6 + 1e-6 * abs(other.x) &&
            abs(self.y - other.y) <= 1e-6 + 1e-6 * abs(other.y) &&
            abs(self.z - other.z) <= 1e-6 + 1e-6 * abs(other.z)  &&
            self.name == other.name
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, other: Node) -> Node {
        Node {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            name: "".into()
        }
    }
}
impl Sub for Node {
    type Output = Node;

    fn sub(self, other: Node) -> Node {
        Node {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            name: "".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn add_two_nodes() {
        let node1 = Node{ x: 0.1, y: 0.2, z: 0.3, name: "N1".into() };
        let node2 = Node{ x: -20.0, y: 0.1, z: -1.0, name: "N2".into() };
        let ref_result = Node{ x: -19.9, y: 0.3, z: -0.7, name: "".into() };
        let result = node1 + node2;
        assert_eq!(result, ref_result);
    }
    #[test]
    fn substract_two_nodes() {
        let node1 = Node{ x: 0.1, y: 0.2, z: 0.3, name: "N1".into() };
        let node2 = Node{ x: -20.0, y: 0.1, z: -1.0, name: "N2".into() };
        let ref_result = Node{ x: 20.1, y: 0.1, z: 1.3, name: "".into() };
        let result = node1 - node2;
        assert_eq!(result, ref_result);
    }
}
