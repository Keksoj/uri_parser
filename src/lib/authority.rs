use nom::{
    self, bytes::complete::tag, character::complete::alphanumeric1, combinator::opt,
    error::context, sequence,
};

use crate::lib::CustomResult;

pub type Authority<'a> = (
    &'a str,         // username
    Option<&'a str>, // optional password
);

pub fn authority_parser(input: &str) -> CustomResult<&str, Authority> {
    context(
        "authority",
        // terminated by @
        sequence::terminated(
            sequence::separated_pair(
                alphanumeric1,      // user
                opt(tag(":")),      // separator
                opt(alphanumeric1), // optional password
            ),
            tag("@"), // the sequence terminator
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_authority_parser() {
        assert_eq!(
            authority_parser("username:password@zupzup.org"),
            Ok(("zupzup.org", ("username", Some("password"))))
        );
        assert_eq!(
            authority_parser("username@zupzup.org"), // input with no authority
            Ok((
                "zupzup.org",       // the very same input
                ("username", None)  // default values
            ))
        );
        assert_eq!(
            authority_parser("zupzup.org"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (".org", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("zupzup.org", VerboseErrorKind::Context("authority")),
                ]
            }))
        );
        assert_eq!(
            authority_parser(":zupzup.org"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (
                        ":zupzup.org",
                        VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)
                    ),
                    (":zupzup.org", VerboseErrorKind::Context("authority")),
                ]
            }))
        );
    }
}
