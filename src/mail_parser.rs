use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{opt, recognize, value},
    multi::{many0, many1},
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

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn node_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = recognize(pair(alpha1, digit1))(input)?;
    Ok((input, name))
}

fn node_3d_coords(input: &str) -> IResult<&str, [f32; 3]> {
    let (input, (_, x, _, y, _, z, _)) = tuple((
        multispace0,
        float,
        multispace1,
        float,
        multispace1,
        float,
        multispace0,
    ))(input)?;

    Ok((input, [x, y, z]))
}

fn node_description(input: &str) -> IResult<&str, Node> {
    let (input, (_, name, _, [x, y, z], _)) = tuple((
        multispace0,
        node_name,
        multispace0,
        node_3d_coords,
        multispace0,
    ))(input)?;
    Ok((input, Node { name, x, y, z }))
}

fn end_section_tag(input: &str) -> IResult<&str, &str> {
    tag("FINSF")(input)
}

fn start_3d_node_section(input: &str) -> IResult<&str, &str> {
    tag("COOR_3D")(input)
}
fn node_3d_section(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, (_, nodes, _)) = tuple((
        start_3d_node_section,
        many0(node_description),
        end_section_tag,
    ))(input)?;
    Ok((input, nodes))
}

#[cfg(test)]
mod tests {

    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn node_description_parser_should_work() {
        assert_debug_snapshot!(node_description("23"));
        assert_debug_snapshot!(node_description("N12"));
        assert_debug_snapshot!(node_description("N12 1.2  23.3 233"));
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
        assert_debug_snapshot!(node_3d_section("COOR_3D  \n\nN1 2  3.0 4\nFINSF"));
        assert_debug_snapshot!(node_3d_section("COOR_3D  \nN1 2  3.0 4\nN2 3  4 4\nFINSF"));
        assert_debug_snapshot!(node_3d_section("COOR_3D  \nN1 2  3.0 4\nBLABLA\nN2 3  4 4\nFINSF"));
    }
}
