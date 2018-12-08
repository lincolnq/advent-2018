use std::io::{self, Read};
mod advent1;
mod advent2;
use advent2::*;

fn main() {
    println!("Starting.");

    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    //println!("advent1: {}", advent1::advent1(str).expect("yo"));
    println!("advent2: {}", advent2(str).expect("yo"));
}
