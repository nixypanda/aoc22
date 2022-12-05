use std::ops::RangeInclusive;

use aoc22::parsers::decimal;
use aoc22::range::{Overlap, Subsume};
use nom::{bytes::complete::tag, Finish};

fn range(input: &str) -> nom::IResult<&str, RangeInclusive<usize>> {
    let (input, start) = decimal(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = decimal(input)?;

    let range = start..=end;
    Ok((input, range))
}

fn range_pair(input: &str) -> nom::IResult<&str, (RangeInclusive<usize>, RangeInclusive<usize>)> {
    let (input, first) = range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, second) = range(input)?;

    Ok((input, (first, second)))
}

fn parse_range_pair(string: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    match range_pair(string).finish() {
        Ok((_remaining, range_pair)) => range_pair,
        _ => panic!("Invalid Range Pair"),
    }
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input.lines().map(parse_range_pair).collect()
}

fn part1(ranges: &[(RangeInclusive<usize>, RangeInclusive<usize>)]) -> usize {
    ranges
        .iter()
        .filter(|(r1, r2)| r1.subsumes(r2) || r2.subsumes(r1))
        .count()
}

fn part2(ranges: &[(RangeInclusive<usize>, RangeInclusive<usize>)]) -> usize {
    ranges.iter().filter(|(r1, r2)| r1.overlaps(r2)).count()
}

fn main() {
    let ranges = parse_input(include_str!("../../data/day04.txt"));
    println!("Day 04 - Part 01: {}", part1(&ranges));
    println!("Day 04 - Part 01: {:?}", part2(&ranges));
}
