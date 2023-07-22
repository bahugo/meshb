use nom::{
  IResult,
  branch::alt,
  multi::{many0, many1},
  number::complete::float,
  combinator::{opt, recognize, value},
  sequence::{preceded, delimited, terminated, tuple, pair},
  character::complete::{char, one_of,multispace0, multispace1, alpha1, digit1 },
};


    //  COOR_3D
    //  N1        1.00000000000000E+00  4.00000000000000E+00  2.50000000000000E+00
    //  N2        2.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    //  N3        3.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    //  N4        4.00000000000000E+00  4.00000000000000E+00  1.50000000000000E+00
    // FINSF

#[derive(Debug)]
pub struct Node<'a>{
    pub name: &'a str,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn node_name(input: &str) -> IResult<&str, &str>{
    let (input, name) = recognize(pair(alpha1, digit1))(input)?;
    Ok((input, name))
}

fn node_3d_coords(input: &str) -> IResult<&str, [f32;3]> {
  let (input, (_, x, _, y, _, z, _)) = tuple((
          multispace0,
      float,
      multispace1,
      float,
      multispace1,
      float,
      multispace0
      )
  )(input)?;

  Ok((input, [x, y, z]))
}

fn node_description(input: &str) -> IResult<&str, Node>{
    let (input, (name, [x, y, z])) = tuple((node_name, node_3d_coords ))(input)?;
    Ok((input, Node{name, x,y,z}))
}

#[cfg(test)]
mod tests{

    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn node_description_parser_should_work() {
        assert_debug_snapshot!(node_description("23"));
        assert_debug_snapshot!(node_description("N12"));
        assert_debug_snapshot!(node_description("N12 1.2  23.3 233"));
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
    }
}
