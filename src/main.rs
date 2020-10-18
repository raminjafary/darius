use std::env;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    // It is just an example and should be deleted!
    parse::parse_args(args)
}
