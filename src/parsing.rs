use std::num::ParseIntError;
use nom::digit;

fn from_str_i32(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(input, 10)
}

named!(pub int32 <&str, i32>,
    map_res!(digit, from_str_i32)
);

