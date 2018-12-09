use nom::{digit, multispace0, multispace1};
use std::str::FromStr;
use std::num::ParseIntError;
use ndarray::Array2;

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

type Board = Array2<i32>;

pub fn advent3(s: String) -> Result<i32, &'static str> {

    let parsed_claims = claims(&s).expect("unable to parse claims").1;

    let mut board = Array2::<i32>::zeros((10, 10));

    for c in parsed_claims.iter() {
        claim_board(&mut board, c);
    }

    println!("board:\n{:?}", board);


    Err("nyi")
}

fn claim_board(b: &mut Board, c: &Claim) {
    let mut slice = b.slice_mut(s![c.x..c.x+c.w, c.y..c.y+c.w]);

    slice.mapv_inplace(|v|
        if v == 0 {
            // no conflict
            c.id
        } else {
            // conflict, return -1
            -1
        }
    )
}