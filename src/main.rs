#[macro_use]
extern crate nom;

use std::io::{self, Read};
mod advent1;
mod advent2;
mod advent3;
use advent2::*;
use advent3::*;


fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    //println!("advent1: {}", advent1::advent1(str).expect("yo"));
    //println!("advent2: {}", advent2(str).expect("yo"));
    println!("advent3: {}", advent3(str).unwrap());
}
