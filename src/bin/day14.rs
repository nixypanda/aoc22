use std::{cmp::Ordering, collections::HashSet};

use aoc22::parsers::decimal;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

type Location = (usize, usize);
type Path = Vec<Location>;

fn point(input: &str) -> IResult<&str, Location> {
    let (input, x) = decimal(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = decimal(input)?;

    Ok((input, (x, y)))
}

fn path(input: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), point)(input)
}

fn parse(input: &str) -> Vec<Path> {
    match separated_list1(tag("\n"), path)(input) {
        Ok((_remaining, paths)) => paths,
        Err(_) => panic!("Bad input"),
    }
}

fn line((start_x, start_y): Location, (end_x, end_y): Location) -> Vec<Location> {
    match (end_x.cmp(&start_x), end_y.cmp(&start_y)) {
        (Ordering::Less, Ordering::Equal) => {
            (end_x..=start_x).into_iter().map(|x| (x, end_y)).collect()
        }
        (Ordering::Greater, Ordering::Equal) => {
            (start_x..=end_x).into_iter().map(|x| (x, end_y)).collect()
        }
        (Ordering::Equal, Ordering::Less) => {
            (end_y..=start_y).into_iter().map(|y| (end_x, y)).collect()
        }
        (Ordering::Equal, Ordering::Greater) => {
            (start_y..=end_y).into_iter().map(|y| (end_x, y)).collect()
        }
        _ => {
            panic!("Incorret line")
        }
    }
}

fn build_reserviour(paths: Vec<Path>) -> HashSet<Location> {
    let mut rocks = HashSet::new();

    for path in paths {
        for location in path
            .iter()
            .tuple_windows()
            .flat_map(|(start, end)| line(*start, *end))
        {
            rocks.insert(location);
        }
    }

    rocks
}

enum StepResult {
    OutOfBounds,
    NoChange,
    Change(Location),
}

enum ParticaleResult {
    Settled(Location),
    IntoTheAbyss,
}

#[derive(Debug)]
enum SimulationResult {
    IntoTheAbyss(usize),
    FilledToBrim(usize),
}

struct Reserviour {
    no_air: HashSet<Location>,
    start: Location,
    min_bound: Location,
    max_bound: Location,
}

impl Reserviour {
    fn new(rocks: HashSet<Location>, start: Location) -> Self {
        let min_x = *rocks.iter().map(|(x, _)| x).min().unwrap();
        let min_y = 0;
        let max_x = *rocks.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

        Reserviour {
            no_air: rocks,
            start,
            min_bound: (min_x, min_y),
            max_bound: (max_x, max_y),
        }
    }

    fn new_with_base(rocks: HashSet<Location>, start: Location) -> Self {
        let min_x = 0;
        let min_y = 0;
        // given that we started at 500 so I just cerated a line from (0, max_y) to (1000, max_y)
        let max_x = 1000;
        let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap() + 2;

        let mut reserviour = Reserviour {
            no_air: rocks,
            start,
            min_bound: (min_x, min_y),
            max_bound: (max_x, max_y),
        };

        for location in line((min_x, max_y), (max_x, max_y)) {
            reserviour.block(&location);
        }

        reserviour
    }

    fn step(&mut self, (start_x, start_y): Location) -> StepResult {
        let new_location = (start_x, start_y + 1);
        if self.out_of_bounds(&new_location) {
            StepResult::OutOfBounds
        } else if self.occupied(&new_location) {
            let new_location = (start_x - 1, start_y + 1);
            if self.out_of_bounds(&new_location) {
                StepResult::OutOfBounds
            } else if self.occupied(&new_location) {
                let new_location = (start_x + 1, start_y + 1);
                if self.out_of_bounds(&new_location) {
                    StepResult::OutOfBounds
                } else if self.occupied(&new_location) {
                    StepResult::NoChange
                } else {
                    StepResult::Change(new_location)
                }
            } else {
                StepResult::Change(new_location)
            }
        } else {
            StepResult::Change(new_location)
        }
    }

    fn block(&mut self, location: &Location) {
        self.no_air.insert(*location);
    }

    fn occupied(&self, location: &Location) -> bool {
        self.no_air.contains(location)
    }

    fn out_of_bounds(&self, (x, y): &Location) -> bool {
        let (min_x, min_y) = self.min_bound;
        let (max_x, max_y) = self.max_bound;

        min_x > *x || max_x < *x || min_y > *y || max_y < *y
    }

    fn simulate_particle(&mut self) -> ParticaleResult {
        let mut location = self.start;
        loop {
            match self.step(location) {
                StepResult::OutOfBounds => return ParticaleResult::IntoTheAbyss,
                StepResult::NoChange => return ParticaleResult::Settled(location),
                StepResult::Change(new_location) => location = new_location,
            }
        }
    }

    fn simulate(&mut self) -> SimulationResult {
        let mut particle_number = 0;
        loop {
            match self.simulate_particle() {
                ParticaleResult::Settled(location) => {
                    if location == self.start {
                        return SimulationResult::FilledToBrim(particle_number);
                    }
                    self.block(&location);
                    particle_number += 1;
                }
                ParticaleResult::IntoTheAbyss => {
                    return SimulationResult::IntoTheAbyss(particle_number)
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../../data/day14.txt");
    let mut reserviour = Reserviour::new(build_reserviour(parse(input)), (500, 0));
    println!("Day 14 - Part 01: {:?}", reserviour.simulate());
    let mut reserviour = Reserviour::new_with_base(build_reserviour(parse(input)), (500, 0));
    println!("Day 14 - Part 02: {:?}", reserviour.simulate());
}
