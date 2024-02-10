use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{
        alpha1, alphanumeric1, digit1, line_ending, multispace0,
        multispace1, space0,
    },
    combinator::{opt, recognize},
    multi::{many0, many1, many_m_n, separated_list0},
    number::complete::float,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use nom_supreme::final_parser::final_parser;
use nom_supreme::ParserExt;
use nom_supreme::{
    error::ErrorTree,
    tag::complete::{tag, tag_no_case},
};

use crate::{
    lib::CellType,
    parsers::tokens::{NodeProp, CellProp, Group, GroupType, MailValue, MailParseOutput},
};

fn node_or_cell_name(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (input, name) = recognize(pair(alpha1, digit1))(input)?;
    Ok((input, name))
}

fn node_3d_coords(input: &str) -> IResult<&str, [f32; 3], ErrorTree<&str>> {
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

fn node_description(input: &str) -> IResult<&str, NodeProp, ErrorTree<&str>> {
    let (input, (_, name, _, [x, y, z], _)) = tuple((
        space0,
        node_or_cell_name,
        space0,
        node_3d_coords,
        multispace0,
    ))(input)?;
    Ok((input, NodeProp { name, x, y, z }))
}

fn cell_description(cell_type: CellType, input: &str) -> IResult<&str, CellProp, ErrorTree<&str>> {
    let nb_nodes: usize = cell_type.get_nb_of_connectivities();
    let (input, (name, node_names, _)) = tuple((
        node_or_cell_name,
        many_m_n(nb_nodes, nb_nodes, preceded(multispace1, node_or_cell_name)),
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

fn group_description(group_type: GroupType, input: &str) -> IResult<&str, Group, ErrorTree<&str>> {
    let (input, (_, _, _, grp_name, _)) = tuple((
        space0,
        opt(tuple((tag_no_case("NOM"), space0, tag("=")))).context("optional NOM ="),
        space0,
        group_name.context("group_name"),
        opt(many0(comment_or_line_ending)),
    ))(input)?;
    let (input, (elems_names, _, _, _)) = tuple((
        many0(preceded(multispace1, node_or_cell_name)).context("elements names"),
        many0(comment_or_line_ending).context("optional space comment or line ending"),
        multispace0,
        end_section_tag,
    ))(input)?;
    Ok((
        input,
        Group {
            group_type,
            name: grp_name,
            elems: elems_names,
        },
    ))
}

fn end_section_tag(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    tag("FINSF")(input)
}

fn start_3d_node_section(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    tag("COOR_3D")(input)
}

fn start_gno_section(input: &str) -> IResult<&str, GroupType, ErrorTree<&str>> {
    let (input, _) = tag("GROUP_NO")(input)?;
    Ok((input, GroupType::Node))
}

fn start_gma_section(input: &str) -> IResult<&str, GroupType, ErrorTree<&str>> {
    let (input, _) = tag("GROUP_MA")(input)?;
    Ok((input, GroupType::Cell))
}

//  COOR_3D
//  N1        1.00000000000000E+00  4.00000000000000E+00  2.50000000000000E+00
//  N2        2.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
//  N3        3.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
//  N4        4.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
// FINSF
fn node_3d_section(input: &str) -> IResult<&str, MailValue, ErrorTree<&str>> {
    let (input, (_, _, nodes, _, _)) = tuple((
        tuple((space0, start_3d_node_section, space0)),
        many1(comment_or_line_ending),
        many0(preceded(multispace0, node_description)),
        many0(comment_or_line_ending),
        end_section_tag,
    ))(input)?;
    Ok((input, MailValue::NodeElts(nodes)))
}

fn cell_type_tag(input: &str) -> IResult<&str, CellType, ErrorTree<&str>> {
    let (input, cell_type_str) = alt((
        tag("POI1"),
        tag("SEG2"),
        tag("SEG3"),
        tag("SEG4"),
        tag("TRIA3"),
        tag("TRIA6"),
        tag("TRIA7"),
        tag("QUAD4"),
        tag("QUAD8"),
        tag("QUAD9"),
        tag("HEXA8"),
        tag("HEXA20"),
        tag("HEXA27"),
        tag("PENTA6"),
        tag("PENTA15"),
        tag("PENTA18"),
        tag("TETRA4"),
        tag("TETRA10"),
        tag("PYRAM5"),
        tag("PYRAM13"),
    ))(input)?;
    let cell_type_resu = CellType::from_string(cell_type_str);
    if let Ok(cell_type) = cell_type_resu {
        return Ok((input, cell_type));
    }
    else {
        panic!("{} not implemented", cell_type_str);
    }
}

fn cell_section(input: &str) -> IResult<&str, MailValue, ErrorTree<&str>> {
    let (input, (_, cell_type, _)) = tuple((space0, cell_type_tag, space0))(input)?;
    let (input, (cells, _, _)) = tuple((
        many0(preceded(multispace0, |input| {
            cell_description(cell_type.clone(), input)
        })),
        many0(comment_or_line_ending),
        end_section_tag,
    ))(input)?;
    Ok((input, MailValue::Cells(cells)))
}

fn group_name(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (input, name) = recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)?;
    Ok((input, name))
}

fn group_section(input: &str) -> IResult<&str, MailValue, ErrorTree<&str>> {
    let (input, (_, group_type, _, _, _)) = tuple((
        space0,
        alt((start_gno_section, start_gma_section)),
        multispace0,
        many0(comment_or_line_ending),
        multispace0,
    ))(input)?;

    let (input, group) = preceded(multispace0, |input| {
        group_description(group_type.clone(), input)
    })(input)?;
    Ok((input, MailValue::Group(group)))
}

fn comment_or_line_ending(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
    let (input, _) = tuple((
        opt(preceded(
            tag("%").context("commentary symbol"),
            is_not("\n").context("not end of line"),
        )),
        line_ending,
    ))(input)?;
    Ok((input, ()))
}

fn useless_line(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
    let (input, _) = preceded(space0, comment_or_line_ending)(input)?;
    Ok((input, ()))
}

fn mail_intermediate_parser(input: &str) -> IResult<&str, MailParseOutput, ErrorTree<&str>> {
    let (input, parsed) = delimited(
        many0(useless_line),
        separated_list0(
            many1(useless_line),
            alt((node_3d_section, cell_section, group_section)),
        ),
        many0(useless_line),
    )(input)?;

    let output =
        parsed.iter().fold(
            MailParseOutput::new(),
            |mut acc: MailParseOutput, item| match item {
                MailValue::NodeElts(nodes) => {
                    acc.nodes.extend(nodes.to_owned());
                    acc
                }
                MailValue::Cells(cells) => {
                    acc.cells.extend(cells.to_owned());
                    acc
                }
                MailValue::Group(group) => {
                    acc.groups.insert(acc.groups.len(), group.to_owned());
                    acc
                }
                _ => acc,
            },
        );
    Ok((input, output))
}

pub fn mail_parser(input: &str) -> Result<MailParseOutput, ()> {
    final_parser(mail_intermediate_parser)(input)
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
        assert_debug_snapshot!(node_or_cell_name("23"));
        assert_debug_snapshot!(node_or_cell_name("N"));
        assert_debug_snapshot!(node_or_cell_name("N12"));
        assert_debug_snapshot!(node_or_cell_name("MD12"));
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
        assert_debug_snapshot!(node_3d_section("COOR_3D\nN1 2  3.0 4\nN2 3  4 4\nFINSF"));
    }

    #[test]
    fn cell_type_tag_should_work() {
        assert_debug_snapshot!(cell_type_tag("POI1"));
        assert_debug_snapshot!(cell_type_tag("Poi1"));
        assert_debug_snapshot!(cell_type_tag("SEG2"));
        assert_debug_snapshot!(cell_type_tag("SEG3"));
        assert_debug_snapshot!(cell_type_tag("SEG4"));
        assert_debug_snapshot!(cell_type_tag("TRIA3"));
        assert_debug_snapshot!(cell_type_tag("TRIA6"));
        assert_debug_snapshot!(cell_type_tag("TRIA7"));
        assert_debug_snapshot!(cell_type_tag("QUAD4"));
        assert_debug_snapshot!(cell_type_tag("QUAD8"));
        assert_debug_snapshot!(cell_type_tag("QUAD9"));
        assert_debug_snapshot!(cell_type_tag("HEXA8"));
        assert_debug_snapshot!(cell_type_tag("HEXA20"));
        assert_debug_snapshot!(cell_type_tag("HEXA27"));
        assert_debug_snapshot!(cell_type_tag("PENTA6"));
        assert_debug_snapshot!(cell_type_tag("PENTA15"));
        assert_debug_snapshot!(cell_type_tag("PENTA18"));
        assert_debug_snapshot!(cell_type_tag("TETRA4"));
        assert_debug_snapshot!(cell_type_tag("TETRA10"));
        assert_debug_snapshot!(cell_type_tag("PYRAM5"));
        assert_debug_snapshot!(cell_type_tag("PYRAM13"));
    }

    #[test]
    fn cell_section_parser_should_work() {
        assert_debug_snapshot!(cell_section("POI1  \n\nM1 N2   \nFINSF"));
        assert_debug_snapshot!(cell_section("POI1  \n\nM1 N2   \nM2 N3\nFINSF"));
        assert_debug_snapshot!(cell_section("POI1  \n\nM1 \n N2   \nM2 N3\nFINSF"));
        assert_debug_snapshot!(cell_section("SEG2\nM1 N1 N2\nFINSF"));
    }
    #[test]
    fn group_name_should_work() {
        assert_debug_snapshot!(group_name("GOP1"));
        assert_debug_snapshot!(group_name("aaaaaaa"));
        assert_debug_snapshot!(group_name("aaa_aaaa"));
        assert_debug_snapshot!(group_name("aa1a_Aaaa"));
        assert_debug_snapshot!(group_name("-a1a_Aaaa"));
        assert_debug_snapshot!(group_name("1a_Aaaa"));
        assert_debug_snapshot!(group_name("GOP2 d"));
    }
    #[test]
    fn start_gno_section_should_work() {
        assert_debug_snapshot!(start_gno_section("GROUP_NO"));
        assert_debug_snapshot!(start_gno_section("GROUPNO"));
    }
    #[test]
    fn start_gma_section_should_work() {
        assert_debug_snapshot!(start_gma_section("GROUP_MA"));
        assert_debug_snapshot!(start_gma_section("GROUPMA"));
    }
    #[test]
    fn group_section_parser_should_work() {
        assert_debug_snapshot!(group_section("GROUP_NO nom = BORD_INT \n bI1 Bi2\n FINSF"));
        assert_debug_snapshot!(group_section("GROUP_NO BORD_INT \n bI1 Bi2\n FINSF"));
        assert_debug_snapshot!(group_section("GROUP_MA nom = BORD_INT \n bI1 Bi2\n FINSF"));
        assert_debug_snapshot!(group_section("GROUP_MA BORD_INT \n bI1 Bi2\n FINSF"));
        assert_debug_snapshot!(group_section("GROUP_MA BORD_INT \nbI1 Bi2 \n FINSF"));
        assert_debug_snapshot!(group_section("GROUP_MA \nBORD_INT bI1 Bi2 \n FINSF"));
    }

    #[test]
    fn comment_or_lineending_should_work() {
        assert_debug_snapshot!(comment_or_line_ending("%ble\n"));
        assert_debug_snapshot!(comment_or_line_ending("% ble &\n"));
        assert_debug_snapshot!(comment_or_line_ending("ddf % ble &\n"));
    }

    #[test]
    fn useless_line_should_work() {
        assert_debug_snapshot!(useless_line(" \t%ble\n"));
        assert_debug_snapshot!(useless_line(" \t  \n"));
        assert_debug_snapshot!(useless_line("\n"));
        assert_debug_snapshot!(useless_line(" \n"));
        assert_debug_snapshot!(useless_line(" % ble\n"));
    }

    #[test]
    fn mail_final_parser_should_work() {
        assert_debug_snapshot!(mail_parser(
            "COOR_3D  \n\nN1 2  3.0 4\nFINSF\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF"
        ));
        assert_debug_snapshot!(mail_parser("COOR_3D  \n\nN1 2  3.0 4\nFINSF\nPOI1\nM1 N1\nFINSF\n\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF"));
        assert_debug_snapshot!(mail_parser(
            "COOR_3D %comment \nN1 2  3.0 4\n    % another comment\nFINSF"
        ));
        assert_debug_snapshot!(mail_parser(
            "COOR_3D  \n\nN1 2  3.0 4\nFINSF\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF\nGROUP_NO GRP1 N1 N2 \nFINSF \n"
        ));
        assert_debug_snapshot!(mail_parser(
            "\nCOOR_3D  \n\nN1 2  3.0 4\nFINSF\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF"
        ));
        assert_debug_snapshot!(mail_parser(
            " \n %comment\nCOOR_3D  \n\nN1 2  3.0 4\nFINSF\nCOOR_3D  \nN2 2  3.0 4\nN3 3  4 4\nFINSF \n %comment\n \n"
        ));
    }
}
