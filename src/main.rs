use std::io::{self, Read};
mod advent1;

fn main() {
    println!("Hello, world!");
    let mut str = String::new();
    io::stdin().read_to_string(&mut str).expect("unable to read stdin");
    //println!("advent1: {}", advent1::advent1(str).expect("yo"));
    println!("advent2: {}", advent2(str).expect("yo"));
}
