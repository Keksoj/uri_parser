use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, one_of},
    error::Error as NomErr,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many1, many_m_n},
    sequence::{terminated, tuple},
    AsChar, IResult,
};

use crate::lib::CustomResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    // a host is either:
    HOST(String), // a string, like "example.com"
    IP([u8; 4]),  // a array of four bytes, parsed from a "185.42.23.3" string
}

// recognize a text host, like "localhost" or "youtube.com"
pub fn host_parser(input: &str) -> CustomResult<&str, Host> {
    context(
        "host",
        alt((
            // within the alt combinator, all options have to return the same type
            // the type here is: (Vec<String>, String), which explains the use of
            // many_m_n(1, 1, _)
            tuple((
                many1(
                    // find many patterns that end with a period
                    terminated(
                        alphanumerichyphen1,
                        tag("."), // bytes::complete::tag
                    ),
                ),
                alpha1, // find one string of letters a-Z (the "com" of ".com")
            )),
            tuple((
                // in case there is no period in the domain name (ex: localhost)
                many_m_n(1, 1, alphanumerichyphen1),
                take(0 as usize), // bytes::complete::take
            )),
        )),
    )(input)
    .map(|(next_input, mut res)| {
        if !res.1.is_empty() {
            res.0.push(res.1);
        }
        (next_input, Host::HOST(res.0.join(".")))
    })
}

// This is a bit complex, but it’s basically just a copied version
// of nom’s alphanumeric1 parser, with the hyphen (-) added.
fn alphanumerichyphen1<T>(i: T) -> CustomResult<T, T>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum()
        },
        ErrorKind::AlphaNumeric,
    )
}

// To get each individual number, we try to find one to three consecutive digits
// using our n_to_m_digits parser and convert them to a u8.
fn ip_number_parser(input: &str) -> CustomResult<&str, u8> {
    context(
        "ip number",
        // this function is defined below
        n_to_m_digits(1, 3),
    )(input)
    .and_then(|(next_input, result)| {
        match result.parse::<u8>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(
                // putting this comment just to split up the returned error
                nom::Err::Error(
                    // I am getting sick of so much encapsulation
                    nom::error::VerboseError {
                        // this is better, innit?
                        errors: vec![],
                    },
                ),
            ),
        }
    })
}

fn n_to_m_digits<'a>(min: usize, max: usize) -> impl FnMut(&'a str) -> CustomResult<&str, String> {
    move |input| {
        many_m_n(min, max, one_of("0123456789"))(input)
            .map(|(next_input, result)| (next_input, result.into_iter().collect()))
    }
}

fn ip(input: &str) -> CustomResult<&str, Host> {
    context(
        "ip",
        tuple(
            //
            (
                count(terminated(ip_number_parser, tag(".")), 3),
                ip_number_parser,
            ),
        ),
    )(input)
    .map(|(next_input, res)| {
        let mut result: [u8; 4] = [0, 0, 0, 0];
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| result[i] = v);
        result[3] = res.1;
        (next_input, Host::IP(result))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_host_parser() {
        assert_eq!(
            host_parser("localhost:8080"),
            Ok((":8080", Host::HOST("localhost".to_string())))
        );
        assert_eq!(
            host_parser("example.org:8080"),
            Ok((":8080", Host::HOST("example.org".to_string())))
        );
        assert_eq!(
            host_parser("some-subsite.example.org:8080"),
            Ok((":8080", Host::HOST("some-subsite.example.org".to_string())))
        );
        assert_eq!(
            host_parser("example.123"),
            Ok((".123", Host::HOST("example".to_string())))
        );
        assert_eq!(
            host_parser("$$$.com"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("$$$.com", VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)),
                    ("$$$.com", VerboseErrorKind::Nom(ErrorKind::ManyMN)),
                    ("$$$.com", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("$$$.com", VerboseErrorKind::Context("host")),
                ]
            }))
        );
        assert_eq!(
            host_parser(".com"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (".com", VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)),
                    (".com", VerboseErrorKind::Nom(ErrorKind::ManyMN)),
                    (".com", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    (".com", VerboseErrorKind::Context("host")),
                ]
            }))
        );
    }
}
