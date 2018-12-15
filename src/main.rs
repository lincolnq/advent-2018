#[macro_use]
extern crate nom;
#[macro_use(s)]
extern crate ndarray;
extern crate chrono;
#[macro_use(iproduct)]
extern crate itertools;
extern crate ordered_float;

use std::io::{self, Read};
mod advent10_stars;
mod parsing;

use advent10_stars::*;


fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    println!("result: {}", advent10(str).unwrap());
}
