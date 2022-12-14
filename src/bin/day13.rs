use aoc22::parsers::decimal;
use nom::multi::separated_list0;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, sequence::delimited, IResult,
};

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Element {
    Base(isize),
    List(Vec<Element>),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Element::Base(lhs), Element::Base(rhs)) => lhs.eq(rhs),
            (Element::List(lhs), Element::List(rhs)) => lhs.eq(rhs),
            (lhs @ Element::List(_), Element::Base(rhs)) => {
                lhs.eq(&Element::List(vec![Element::Base(*rhs)]))
            }
            (Element::Base(lhs), rhs @ Element::List(_)) => {
                Element::List(vec![Element::Base(*lhs)]).eq(rhs)
            }
        }
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Element::Base(lhs), Element::Base(rhs)) => lhs.partial_cmp(rhs),
            (Element::List(lhs), Element::List(rhs)) => {
                for zipped in lhs.iter().zip_longest(rhs.iter()) {
                    match zipped {
                        Both(lhs_element, rhs_element) => {
                            let cmp = lhs_element.partial_cmp(rhs_element).unwrap();
                            if cmp != Ordering::Equal {
                                return Some(cmp);
                            }
                        }
                        Left(_) => return Some(Ordering::Greater),
                        Right(_) => return Some(Ordering::Less),
                    }
                }
                Some(Ordering::Equal)
            }
            (lhs @ Element::List(_), Element::Base(rhs)) => {
                lhs.partial_cmp(&Element::List(vec![Element::Base(*rhs)]))
            }
            (Element::Base(lhs), rhs @ Element::List(_)) => {
                Element::List(vec![Element::Base(*lhs)]).partial_cmp(rhs)
            }
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn base(input: &str) -> IResult<&str, Element> {
    let (input, base) = decimal(input)?;
    Ok((input, Element::Base(base as isize)))
}

fn list(input: &str) -> IResult<&str, Element> {
    delimited(
        char('['),
        separated_list0(tag(","), alt((list, base))),
        char(']'),
    )(input)
    .map(|(input, list)| (input, Element::List(list)))
}

fn parse(input: &str) -> Vec<Element> {
    match separated_list0(alt((tag("\n\n"), tag("\n"))), list)(input) {
        Ok((_remaining, pairs)) => pairs,
        Err(e) => panic!("{:?}", e),
    }
}

fn part1(list: &[Element]) -> usize {
    list.iter()
        .tuples::<(_, _)>()
        .enumerate()
        .filter(|(_, (first, second))| first < second)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

fn part2(list: &mut Vec<Element>) -> usize {
    let marker1 = Element::List(vec![Element::List(vec![Element::Base(2)])]);
    let marker2 = Element::List(vec![Element::List(vec![Element::Base(6)])]);

    list.push(marker1.clone());
    list.push(marker2.clone());

    list.iter()
        .sorted()
        .enumerate()
        .filter(|(_, element)| **element == marker1 || **element == marker2)
        .map(|(i, _)| i + 1)
        .product::<usize>()
}

fn main() {
    let input = include_str!("../../data/day13.txt");
    let mut parsed = parse(input);
    println!("Day 13 - Part 01: {}", part1(&parsed));
    println!("Day 13 - Part 02: {}", part2(&mut parsed));
}
