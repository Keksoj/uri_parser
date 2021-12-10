# Learn nom with Mario Zupan's tutorial

[Here is the tutorial](https://blog.logrocket.com/parsing-in-rust-with-nom/) that teaches how to use nom to parse URIs.

The goal is to learn:

-   how to write simple parsers (recognize an `http://` prefix for instance)
-   how to combine the simple parsers into complex ones. For instance, a host may be a domain name, or an IP address. How do you go around that?

I took the liberty of renaming of few things, like `Res` that I'd rather call `CustomResult`, and to reorder the code into modules.
