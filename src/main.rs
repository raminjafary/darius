#![allow(dead_code)]

mod c_error;
//use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();

    c_error::throw_or("i").unwrap();
}
