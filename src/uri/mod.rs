//! This example library is an example URI parser to better understand
//! how [nom](https://github.com/Geal/nom) works.
//!
//! The goal is to parse such URIs:
//!
//! > https://www.zupzup.org/about/?someVal=5&anotherVal=hello#anchor
//!
//! > http://user:pw@127.0.0.1:8080
//!
//! into a custom URI struct:
//!
//! ```
//! #[derive(Debug, PartialEq, Eq)]
//!pub struct URI<'a> {
//!    scheme: Scheme,
//!    authority: Option<Authority<'a>>,
//!    host: Host,
//!    port: Option<u16>,              
//!    path: Option<Vec<&'a str>>,       
//!    query: Option<QueryParams<'a>>,   
//!    fragment: Option<&'a str>,
//!}
//! ```
pub mod authority;
pub mod host;
pub mod path;
pub mod port;
pub mod query;
pub mod scheme;

use authority::{authority_parser, Authority};
use host::{ip_or_hostname_parser, Host};
use path::path_parser;
use port::port_parser;
use query::{fragment_parser, query_params_parser, QueryParam};
use scheme::{scheme_parser, Scheme};

use nom::{
    self,
    combinator::opt,
    error::{context, VerboseError},
    sequence::tuple,
    IResult,
};

// our custom result wraps nom's VerboseError. VerboseError allows to aggregate
// Errors with a combinator called "context". I like the sound of it.
pub type CustomResult<I, O> = IResult<I, O, VerboseError<I>>;

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,                     // http / https
    authority: Option<Authority<'a>>,   // the optional "user:passowrd@" thing
    host: Host,                         // example.org, or an IPv4
    port: Option<u16>,                  // optional ":8080"
    path: Option<Vec<&'a str>>,         // optional "/user/login"
    query: Option<Vec<QueryParam<'a>>>, // optional "?user=SomeUser&sortBy=newest"
    fragment: Option<&'a str>,          // optional "#inner-link"
}

pub fn uri_parser(input: &str) -> CustomResult<&str, URI> {
    context(
        "uri",
        tuple((
            scheme_parser,
            opt(authority_parser),
            ip_or_hostname_parser,
            opt(port_parser),
            opt(path_parser),
            opt(query_params_parser),
            opt(fragment_parser),
        )),
    )(input)
    .map(|(next_input, res)| {
        let (scheme, authority, host, port, path, query, fragment) = res;
        (
            next_input,
            URI {
                scheme,
                authority,
                host,
                port,
                path,
                query,
                fragment,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_parser() {
        assert_eq!(
            uri_parser("http://localhost"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTP,
                    authority: None,
                    host: Host::HOST("localhost".to_string()),
                    port: None,
                    path: None,
                    query: None,
                    fragment: None
                }
            ))
        );

        assert_eq!(
            uri_parser("https://spongebob:heypatrick@en.wikipedia.org:65000/some/path?key=value&other-key=other-value#frag"),
            Ok(("",URI {
                scheme: Scheme::HTTPS,
                authority: Some(Authority { user: "spongebob", password: Some("heypatrick") }),
                host: Host::HOST("en.wikipedia.org".to_string()),
                port: Some(65_000 as u16),
                path: Some(vec!["some", "path"]),
                query: Some(
                    vec!(

                        QueryParam { key: "key", value: "value"},
                        QueryParam { key: "other-key", value: "other-value"},
                    )
                ),
                fragment: Some("frag"),
            }))
        );

        assert_eq!(
            uri_parser("http://user:pw@127.0.0.1:8080"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTP,
                    authority: Some(Authority {
                        user: "user",
                        password: Some("pw")
                    }),
                    host: Host::IP([127, 0, 0, 1]),
                    port: Some(8080),
                    path: None,
                    query: None,
                    fragment: None
                }
            ))
        );
    }
}
