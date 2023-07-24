use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{
        alpha1, anychar, char, digit1, line_ending, multispace0, multispace1, newline, one_of,
        space0, space1,
    },
    combinator::{opt, recognize, value},
    multi::{fold_many0, many0, many1, many_m_n},
    number::complete::float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

//  COOR_3D
//  N1        1.00000000000000E+00  4.00000000000000E+00  2.50000000000000E+00
//  N2        2.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
//  N3        3.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
//  N4        4.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
// FINSF
//

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    POI1,
    SEG2,
}

#[derive(Debug, PartialEq)]
pub enum MailValue<'a> {
    Null,
    NodeElts(Vec<Node<'a>>),
    Cells(Vec<CellProp<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
struct MailParseOutput<'a> {
    pub nodes: Vec<Node<'a>>,
    pub cells: Vec<CellProp<'a>>,
}

impl MailParseOutput<'_> {
    fn new() -> Self {
        MailParseOutput {
            nodes: vec![],
            cells: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    pub name: &'a str,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellProp<'a> {
    pub cell_type: CellType,
    pub name: &'a str,
    pub nodes: Vec<&'a str>,
}

fn node_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = recognize(pair(alpha1, digit1))(input)?;
    Ok((input, name))
}

fn cell_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = recognize(pair(alpha1, digit1))(input)?;
    Ok((input, name))
}

fn node_3d_coords(input: &str) -> IResult<&str, [f32; 3]> {
    let (input, (_, x, _, y, _, z, _)) = tuple((
        space0,
        float,
        multispace1,
        float,
        multispace1,
        float,
        space0,
    ))(input)?;

    Ok((input, [x, y, z]))
}

fn node_description(input: &str) -> IResult<&str, Node> {
    let (input, (_, name, _, [x, y, z], _)) = tuple((
        space0,
        node_name,
        space0,
        node_3d_coords,
        multispace0,
    ))(input)?;
    Ok((input, Node { name, x, y, z }))
}

fn cell_description(cell_type: CellType, input: &str) -> IResult<&str, CellProp> {
    let nb_nodes: usize = match cell_type {
        CellType::POI1 => 1,
        CellType::SEG2 => 2,
    };
    let (input, (name, node_names, _)) = tuple((
        cell_name,
        many_m_n(nb_nodes, nb_nodes, preceded(multispace1, node_name)),
        multispace0,
    ))(input)?;
    Ok((
        input,
        CellProp {
            cell_type,
            name,
            nodes: node_names,
        },
    ))
}

fn end_section_tag(input: &str) -> IResult<&str, &str> {
    tag("FINSF")(input)
}

fn start_3d_node_section(input: &str) -> IResult<&str, &str> {
    tag("COOR_3D")(input)
}

fn node_3d_section(input: &str) -> IResult<&str, MailValue> {
    let (input, (_, _, nodes, _, _, _)) = tuple((
        tuple((space0, start_3d_node_section, space0)),
        many1(line_ending),
        many0(preceded(multispace0, node_description)),
        many0(line_ending),
        end_section_tag,
        many0(line_ending),
    ))(input)?;
    Ok((input, MailValue::NodeElts(nodes)))
}

fn poi1_section(input: &str) -> IResult<&str, MailValue> {
    let (input, (_, cells, _, _, _)) = tuple((
        tuple((space0, tag("POI1"), space0)),
        many0(preceded(multispace0, |input| {
            cell_description(CellType::POI1, input)
        })),
        many0(line_ending),
        end_section_tag,
        many0(line_ending),
    ))(input)?;
    Ok((input, MailValue::Cells(cells)))
}

fn mail_final_parser(input: &str) -> IResult<&str, MailParseOutput> {
    let mail_elt_parser = preceded(multispace0, alt((node_3d_section, poi1_section)));
    let (input, output) = fold_many0(
        mail_elt_parser,
        MailParseOutput::new,
        |mut acc: MailParseOutput, item| match item {
            MailValue::NodeElts(nodes) => {
                acc.nodes.extend(nodes);
                acc
            }
            MailValue::Cells(cells) => {
                acc.cells.extend(cells);
                acc
            }
            _ => acc,
        },
    )(input)?;
    Ok((input, output))
}

#[cfg(test)]
mod tests {

    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn node_description_parser_should_work() {
        assert_debug_snapshot!(node_description("23"));
        assert_debug_snapshot!(node_description("N12"));
        assert_debug_snapshot!(node_description("N12 1.2  23.3 233\n"));
        assert_debug_snapshot!(node_description("  N12 1.2  23.3 233\n"));
    }

    #[test]
    fn node_name_parser_should_work() {
        assert_debug_snapshot!(node_name("23"));
        assert_debug_snapshot!(node_name("N"));
        assert_debug_snapshot!(node_name("N12"));
        assert_debug_snapshot!(node_name("MD12"));
    }
    #[test]
    fn should_parse_node_3d_coords() {
        assert_debug_snapshot!(node_3d_coords("1 2 3"));
        assert_debug_snapshot!(node_3d_coords("   1  2 3 "));
        assert_debug_snapshot!(node_3d_coords("   1.0  2 3.01 "));
        assert_debug_snapshot!(node_3d_coords("   1.0e1  2E+1 3.01E+00 "));
        assert_debug_snapshot!(node_3d_coords(" 1,2  23.3 233"));
        assert_debug_snapshot!(node_3d_coords("   1\n  2 3 "));
    }
    #[test]
    fn end_section_parser_should_work() {
        assert_debug_snapshot!(end_section_tag("FINSF"));
        assert_debug_snapshot!(end_section_tag("TINSF"));
    }
    #[test]
    fn start_node_section_parser_should_work() {
        assert_debug_snapshot!(start_3d_node_section("COOR_3D"));
        assert_debug_snapshot!(start_3d_node_section("COORD3D"));
    }
    #[test]
    fn node_3d_section_parser_should_work() {
        assert_debug_snapshot!(node_3d_section("COOR_3D  \n\nN1 2  3.0 4\nFINSF\n"));
        assert_debug_snapshot!(node_3d_section("COOR_3D\nN1 2  3.0 4\nN2 3  4 4\nFINSF\n"));
    }
    #[test]
    fn poi1_section_parser_should_work() {
        assert_debug_snapshot!(poi1_section("POI1  \n\nM1 N2   \nFINSF\n"));
        assert_debug_snapshot!(poi1_section("POI1  \n\nM1 N2   \nM2 N3\nFINSF\n"));
        assert_debug_snapshot!(poi1_section("POI1  \n\nM1 \n N2   \nM2 N3\nFINSF\n"));
    }
    #[test]
    fn mail_final_parser_should_work() {
        assert_debug_snapshot!(mail_final_parser(
            "COOR_3D  \n\nN1 2  3.0 4\nFINSF\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF"
        ));
        assert_debug_snapshot!(mail_final_parser("COOR_3D  \n\nN1 2  3.0 4\nFINSF\nPOI1\nM1 N1\nFINSF\n\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF"));
    }
}
