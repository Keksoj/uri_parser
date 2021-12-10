mod lib;

use lib::{
    authority::{authority_parser, Authority},
    host::Host,
    scheme::{scheme_parser, Scheme},
};

fn main() {
    match scheme_parser("bla://yay") {
        Ok((unparsed, parsed)) => println!("{},{:?}", unparsed, parsed),
        Err(e) => println!("{:#?}", e),
    }
}
