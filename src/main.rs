#[macro_use]
extern crate nom;
#[macro_use(s)]
extern crate ndarray;
extern crate chrono;
#[macro_use(iproduct)]
extern crate itertools;

use std::io::{self, Read};
mod advent8_nodetree;
mod parsing;

use advent8_nodetree::*;


fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    println!("result: {}", advent8(str).unwrap());
}
