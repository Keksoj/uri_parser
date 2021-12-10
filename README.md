# Learn nom with [Mario Zupan](https://blog.logrocket.com/author/mariozupan/)'s tutorial

[Nom](https://github.com/Geal/nom) is a library to combine Rust parsers in a versatile-but-not-so-obvious way.

```rust
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
```

It allows for pretty clean code, but to understand how to write such code, there is no better way than to practice.
This repo is meant as a big example in how to use nom.

[Here is the tutorial](https://blog.logrocket.com/parsing-in-rust-with-nom/).

The goal is to learn:

-   how to write simple parsers (recognize an `http://` prefix for instance)
-   how to combine simple parsers into complex ones. For instance, the host part of an URI may be a domain name, or an IP address. How do you go around that?

I took the liberty of changing of few things:

-   rename `Res` that I'd rather call `CustomResult`, and function names
-   arrange the code into modules for clarity
-   rewrite some tuple types into structs, for clarity

Plus, it turns out Mario forgot the details of the port parser in the tutorial.
I did my best to fill in the gap:

```rust
pub fn port_parser(input: &str) -> CustomResult<&str, u16> {
    context(
        "port",
        tuple((
            // find the beginning of the port field
            tag(":"),
            // this combinator gives the output of one parser to the other
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
```

The best to learn, in my not-so-humble opinion, is to retype everything.
So… just rewrite the tutorial in your own repository like I did!

## Tests!

Definitely check out the tests, they show how testable nom is.

    cargo test

## It works!

    cargo run

## Many thanks to Mario Zupan!
