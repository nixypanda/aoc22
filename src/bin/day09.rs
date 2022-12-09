use std::collections::HashSet;

use aoc22::parsers::decimal;
use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0};

type Location = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
struct WeightedDirection {
    direction: Direction,
    weight: usize,
}

fn direction(input: &str) -> nom::IResult<&str, Direction> {
    alt((
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
        map(tag("U"), |_| Direction::Top),
        map(tag("D"), |_| Direction::Bottom),
    ))(input)
}

fn weighted_direction(input: &str) -> nom::IResult<&str, WeightedDirection> {
    let (input, direction) = direction(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, weight) = decimal(input)?;
    let wd = WeightedDirection { direction, weight };

    Ok((input, wd))
}

fn parse(input: &str) -> Vec<WeightedDirection> {
    match separated_list0(tag("\n"), weighted_direction)(input) {
        Ok((_remaining, wd)) => wd,
        Err(e) => panic!("{:?}", e),
    }
}

fn move_in_direction((start_x, start_y): Location, direction: Direction) -> Location {
    match direction {
        Direction::Left => (start_x - 1, start_y),
        Direction::Right => (start_x + 1, start_y),
        Direction::Top => (start_x, start_y + 1),
        Direction::Bottom => (start_x, start_y - 1),
    }
}

struct Rope {
    knots: Vec<Location>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            knots: vec![(0, 0); length],
        }
    }

    fn length(&self) -> usize {
        self.knots.len()
    }

    fn move_head(&mut self, direction: Direction) {
        self.knots[0] = move_in_direction(self.knots[0], direction);
        for index in 1..self.knots.len() {
            self.update_knot(index);
        }
    }

    fn update_knot(&mut self, index: usize) {
        let (head_x, head_y) = self.knots[index - 1];
        let (tail_x, tail_y) = self.knots[index];
        let (dx, dy) = (head_x - tail_x, head_y - tail_y);
        let is_far = dx.abs() > 1 || dy.abs() > 1;

        let (tdx, tdy) = if is_far {
            (dx.signum(), dy.signum())
        } else {
            (0, 0)
        };

        self.knots[index] = (tail_x + tdx, tail_y + tdy)
    }

    fn tail_location(&self, index: usize) -> Location {
        self.knots[index]
    }
}

fn unique_locations_visited_by_tail(rope: &mut Rope, directions: &[WeightedDirection]) -> usize {
    let tail_index = rope.length() - 1;

    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert((0, 0));

    for WeightedDirection { direction, weight } in directions {
        for _step in 0..*weight {
            rope.move_head(*direction);
            visited_by_tail.insert(rope.tail_location(tail_index));
        }
    }

    visited_by_tail.len()
}

fn part1(directions: &[WeightedDirection]) -> usize {
    let mut rope = Rope::new(2);
    unique_locations_visited_by_tail(&mut rope, directions)
}

fn part2(directions: &[WeightedDirection]) -> usize {
    let mut rope = Rope::new(10);
    unique_locations_visited_by_tail(&mut rope, directions)
}

fn main() {
    let str = include_str!("../../data/day09.txt");
    let directions = parse(str);

    println!("{}", part1(&directions));
    println!("{}", part2(&directions));
}
