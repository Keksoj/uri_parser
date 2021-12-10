mod uri;

use uri::uri_parser;

fn main() {
    let uri_to_parse = "https://spongebob:heypatrick@en.wikipedia.org:65000/some/path?key=value&other-key=other-value#frag";

    println!("We want to parse this uri:\n{}", uri_to_parse);

    println!("Here it is: {:#?}", uri_parser(uri_to_parse));
}
