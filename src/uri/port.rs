use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, error::context,
    sequence::tuple,
};

use crate::uri::CustomResult;

pub fn port_parser(input: &str) -> CustomResult<&str, u16> {
    context(
        "port",
        tuple((
            // find the beginning of the port field
            tag(":"),
            //  parse_a_u16
            map_res(
                digit1,            // recognize ASCII chars from 0 to 9
                str::parse::<u16>, // parse them into u16
            ),
        )),
    )(input)
    // the value returned is ("next_input", (":", 8080))
    // we got to extract the port from this inner tuple
    .map(|(next_input, res)| (next_input, res.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_parser() {
        assert_eq!(port_parser(":8080"), Ok(("", 8080u16)));
        assert_eq!(port_parser(":60"), Ok(("", 60u16)));
    }
}
