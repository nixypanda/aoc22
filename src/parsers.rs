use nom::combinator::map_res;
use nom::{character::complete::digit1, character::complete::i32, combinator::map, IResult};

pub fn decimal(input: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub fn signed_decimal(input: &str) -> IResult<&str, isize> {
    // Parse a signed integer using the `signed` combinator
    map(i32, |n| n as isize)(input)
}
