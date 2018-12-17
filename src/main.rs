#[macro_use]
extern crate nom;
#[macro_use(s)]
extern crate ndarray;
extern crate chrono;
#[macro_use(iproduct)]
extern crate itertools;
extern crate ordered_float;

use std::io::{self, Read};
mod advent11_convolve_power_level;
mod parsing;

use advent11_convolve_power_level::*;


fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    println!("result: {}", advent11(str).unwrap());
}
