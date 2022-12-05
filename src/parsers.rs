use nom::{character::complete::digit1, combinator::map_res};

pub fn decimal(input: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}
