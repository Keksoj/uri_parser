use nom::{self, bytes::complete::tag_no_case, error::context};

use crate::uri::CustomResult;

/// The scheme is the beginning of a URI, either http or https
#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl From<&str> for Scheme {
    fn from(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("No other schemes supported"),
        }
    }
}

pub fn scheme_parser(
    input: &str, // the input will be an URI
) -> CustomResult<
    &str,   // the rest of the unparsed URI, same type as the input
    Scheme, // the scheme of the URI
> {
    context(
        "scheme parsing error",
        nom::branch::alt((
            tag_no_case("HTTP://"),  // either one of those parser is chosen
            tag_no_case("HTTPS://"), // tag_no_case means: "parse"
        )),
    )(input)
    // this line is just here to convert "http" into Sheme::HTTP
    .map(|(next_input, scheme_str)| (next_input, scheme_str.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_scheme_parser() {
        assert_eq!(scheme_parser("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme_parser("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(
            scheme_parser("bla://yay"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    (
                        "bla://yay",
                        VerboseErrorKind::Context("scheme parsing error")
                    ),
                ]
            }))
        );
    }
}
