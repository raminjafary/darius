use std::env;
mod parse;
use parse::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    // It is just an example and should be deleted!
    parse_args(args);
}
