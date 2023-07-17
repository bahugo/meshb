use nom::{
  IResult,
  bytes::complete::tag,
  branch::alt,
  multi::{many0, many1},
  number::complete::float,
  combinator::{opt, recognize, value},
  sequence::{preceded, delimited, terminated, tuple},
  character::complete::{char, one_of,multispace0, multispace1 },
};


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

#[cfg(test)]
mod tests{

    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn should_parse_node_3d_coords() {
        assert_debug_snapshot!(node_3d_coords("1 2 3"));
        assert_debug_snapshot!(node_3d_coords("   1  2 3 "));
        assert_debug_snapshot!(node_3d_coords("   1.0  2 3.01 "));

    }
}
