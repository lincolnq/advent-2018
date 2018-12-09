use nom::{digit, multispace0, multispace1};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug,PartialEq)]
pub struct Claim {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

fn from_str_i32(input: &str) -> Result<i32, ParseIntError> {
  i32::from_str_radix(input, 10)
}


named!(hash<&str, char>, char!('#'));
named!(atsign<&str, char>, char!('@'));
named!(comma<&str, char>, char!(','));
named!(colon<&str, char>, char!(':'));
named!(xchar<&str, char>, char!('x'));
named!(int32 <&str, i32>,
    map_res!(digit, from_str_i32)
);
named!(claim <&str, Claim>,
    do_parse!(
        hash >>
        id: int32 >>
        multispace0 >>
        atsign >>
        multispace0 >>
        x: int32 >>
        multispace0 >>
        comma >>
        multispace0 >>
        y: int32 >>
        multispace0 >>
        colon >>
        multispace0 >>
        w: int32 >>
        multispace0 >>
        xchar >>
        multispace0 >>
        h: int32 >>
        (Claim { id, x, y, w, h })
    )
);
named!(claims <&str, Vec<Claim>>,
    separated_list_complete!(multispace1, claim)
);

pub fn advent3(s: String) -> Result<i32, &'static str> {
    //let lines = s.split("\n").filter(|&l| l != "").collect::<Vec<&str>>();
    //println!("digit is {:?}", int32("123\n"));

    let res = claims(&s);
    println!("result is {:?}", res);

    Err("nyi")
}
