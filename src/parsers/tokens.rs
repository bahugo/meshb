use crate::lib::CellType;

#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Node,
    Cell,
}

#[derive(Debug, PartialEq)]
pub enum MailValue<'a> {
    NodeElts(Vec<NodeProp<'a>>),
    Cells(Vec<CellProp<'a>>),
    Group(Group<'a>),
    Title(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MailParseOutput<'a> {
    pub nodes: Vec<NodeProp<'a>>,
    pub cells: Vec<CellProp<'a>>,
    pub groups: Vec<Group<'a>>,
}

impl MailParseOutput<'_> {
    pub fn new() -> Self {
        MailParseOutput {
            nodes: vec![],
            cells: vec![],
            groups: vec![],
        }
    }
}

impl Default for MailParseOutput<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeProp<'a> {
    pub name: Option<&'a str>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellProp<'a> {
    pub cell_type: CellType,
    pub name: Option<&'a str>,
    pub nodes: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group<'a> {
    pub group_type: GroupType,
    pub name: &'a str,
    pub elems: Vec<&'a str>,
}

