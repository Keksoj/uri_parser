// https://blog.logrocket.com/parsing-in-rust-with-nom/
// The goal is to parse uris, like those:
// https://www.zupzup.org/about/?someVal=5&anotherVal=hello#anchor
// or
// http://user:pw@127.0.0.1:8080,
pub mod authority;
pub mod scheme;
pub mod host;

use host::Host;
use authority::Authority;
use scheme::Scheme;

use nom::{self, error::VerboseError, IResult};

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>, // the optional "user:passowrd@" thing
    host: Host,                       // example.org
    port: Option<u16>,                // optional :8080
    path: Option<Vec<&'a str>>,       // optional "/user/login"
    query: Option<QueryParams<'a>>,   // optional "?user=SomeUser&sortBy=newest"
    fragment: Option<&'a str>,
}

pub type QueryParam<'a> = (&'a str, &'a str); // a tuple. ("param=value")
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

// our custom result wraps nom's VerboseError. VerboseError allows to aggregate
// Errors with a combinator called "context". I like the sound of it.
pub type CustomResult<I, O> = IResult<I, O, VerboseError<I>>;
