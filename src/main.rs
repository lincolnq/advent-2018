#[macro_use]
extern crate nom;
#[macro_use(s)]
extern crate ndarray;
extern crate chrono;
#[macro_use(iproduct)]
extern crate itertools;

use std::io::{self, Read};
mod advent1;
mod advent2;
mod advent3;
mod advent4;
mod advent5;
mod advent6;
mod parsing;

use advent2::*;
use advent3::*;
use advent4::*;
use advent5::*;
use advent6::*;


fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    //println!("advent1: {}", advent1::advent1(str).expect("yo"));
    //println!("advent2: {}", advent2(str).expect("yo"));
    //println!("advent3: {}", advent3(str).unwrap());
    //println!("advent4: {}", advent4(str).unwrap());
    //println!("advent5: {}", advent5(str).unwrap());
    println!("advent6: {}", advent6(str).unwrap());
}
