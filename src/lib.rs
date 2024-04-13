extern crate core;

mod mesh;
mod mesh_enums;
mod node;
mod cell;
mod parsers;

pub mod lib{
    pub use crate::mesh::Mesh;
    pub use crate::mesh_enums::{CellType, MeshFormat};
    pub use crate::node::Node;
    pub use crate::cell::MeshCell;
    pub use crate::parsers::mail_parser;
}

