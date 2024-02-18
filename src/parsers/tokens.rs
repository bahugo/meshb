use crate::lib::CellType;

#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Node,
    Cell,
}

#[derive(Debug, PartialEq)]
pub enum MailValue {
    NodeElts(Vec<NodeProp>),
    Cells(Vec<CellProp>),
    Group(Group),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MailParseOutput {
    pub nodes: Vec<NodeProp>,
    pub cells: Vec<CellProp>,
    pub groups: Vec<Group>,
}

impl MailParseOutput {
    pub fn new() -> Self {
        MailParseOutput {
            nodes: vec![],
            cells: vec![],
            groups: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeProp {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl NodeProp {
    pub(crate) fn new(name: &str, x: f32, y: f32, z: f32) -> Self {
        NodeProp {
            name: name.to_owned(),
            x,
            y,
            z,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellProp {
    pub cell_type: CellType,
    pub name: String,
    pub nodes: Vec<String>,
}
impl CellProp {
    pub(crate) fn new(cell_type: CellType, name: &str, nodes: Vec<&str>) -> Self {
        CellProp{
            cell_type,
            name: name.to_owned(),
            nodes: nodes.iter().map(|x| x.to_string()).collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub group_type: GroupType,
    pub name: String,
    pub elems: Vec<String>,
}

impl Group {
    pub(crate) fn new(group_type: GroupType, name: &str, elems: Vec<&str>) -> Self {
        Group {
            group_type,
            name: name.to_owned(),
            elems: elems.iter().map(|x| x.to_string()).collect(),
        }
    }
}
