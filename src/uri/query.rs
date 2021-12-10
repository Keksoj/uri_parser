use nom::{bytes::complete::tag, error::context, multi::many0, sequence::tuple};

use crate::uri::{path::url_code_points, CustomResult};

#[derive(Debug, PartialEq, Eq)]
pub struct QueryParam<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> QueryParam<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        QueryParam { key, value }
    }
}

// converts "?bla=5&blub=val#yay" into a vector of QueryParam
// vec![
//     QueryParam { key: "bla", value: "5" },
//     QueryParam { key: "blub", value: "val" }
// ]
pub fn query_params_parser(input: &str) -> CustomResult<&str, Vec<QueryParam>> {
    context(
        "query params",
        tuple((
            tag("?"),
            url_code_points, // index 1 in the produced tuple
            tag("="),
            url_code_points, // index 3
            many0(
                // index 4, this will be a vector of tuples such as: ("&", str, "=", str)
                tuple((tag("&"), url_code_points, tag("="), url_code_points)),
            ),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut query_params = Vec::new();

        query_params.push(QueryParam::new(res.1, res.3));

        for qp in res.4 {
            query_params.push(QueryParam::new(qp.1, qp.3));
        }
        (next_input, query_params)
    })
}

// This parses the fragment, the #inner-link at the end of an URI
pub fn fragment_parser(input: &str) -> CustomResult<&str, &str> {
    context(
        "fragment",
        // create a tuple of the form ("#", ("", "fragment"))
        tuple((
            tag("#"),        // detect the start of the fragment
            url_code_points, // get the line that follows
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_params_parser() {
        assert_eq!(
            query_params_parser("?bla=5&blub=val#yay"),
            Ok((
                "#yay",
                vec![QueryParam::new("bla", "5"), QueryParam::new("blub", "val")]
            ))
        );

        assert_eq!(
            query_params_parser("?bla-blub=arr-arr#yay"),
            Ok(("#yay", vec![QueryParam::new("bla-blub", "arr-arr")]))
        );
    }

    #[test]
    fn test_fragment_parser() {
        assert_eq!(fragment_parser("#bla"), Ok(("", "bla")));
        assert_eq!(fragment_parser("#bla-blub"), Ok(("", "bla-blub")));
    }
}
