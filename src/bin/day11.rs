use itertools::Itertools;
use std::{collections::VecDeque, u128};

use aoc22::{itertools::Lcm, parsers::decimal};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::map,
    multi::separated_list0, IResult,
};

#[derive(Debug)]
enum Operation {
    Mult(u128),
    Add(u128),
    Pow,
}

impl Operation {
    fn updated_worry_level(&self, worry_level: u128) -> u128 {
        match self {
            Operation::Mult(m) => worry_level * m,
            Operation::Add(a) => worry_level + a,
            Operation::Pow => worry_level * worry_level,
        }
    }
}

#[derive(Debug)]
struct IfThenElse {
    divisible_by: u128,
    case_success: usize,
    case_failure: usize,
}

impl IfThenElse {
    fn throw_to(&self, worry_level: u128) -> usize {
        if worry_level % self.divisible_by == 0 {
            self.case_success
        } else {
            self.case_failure
        }
    }
}

#[derive(Debug)]
struct Note {
    monkey_index: usize,
    items: Vec<u128>,
    operation: Operation,
    test: IfThenElse,
}

fn monkey(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, index) = decimal(input)?;
    let (input, _) = tag(":\n")(input)?;

    Ok((input, index))
}

fn start_items(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list0(tag(", "), map(decimal, |d| d as u128))(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, items))
}

fn mult(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("*")(input)?;
    let (input, _) = space0(input)?;
    let (input, op) = alt((
        map(decimal, |d| Operation::Mult(d as u128)),
        map(tag("old"), |_| Operation::Pow),
    ))(input)?;

    Ok((input, op))
}

fn add(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("+")(input)?;
    let (input, _) = space0(input)?;
    let (input, val) = decimal(input)?;

    Ok((input, Operation::Add(val as u128)))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op) = alt((mult, add))(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, op))
}

fn if_then_else(input: &str) -> IResult<&str, IfThenElse> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, divisible_by) = map(decimal, |d| d as u128)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, case_success) = decimal(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, case_failure) = decimal(input)?;
    let (input, _) = tag("\n")(input)?;

    let if_then_else = IfThenElse {
        divisible_by,
        case_success,
        case_failure,
    };

    Ok((input, if_then_else))
}

fn note(input: &str) -> IResult<&str, Note> {
    let (input, monkey_index) = monkey(input)?;
    let (input, items) = start_items(input)?;
    let (input, operation) = operation(input)?;
    let (input, test) = if_then_else(input)?;

    let entry = Note {
        monkey_index,
        items,
        operation,
        test,
    };

    Ok((input, entry))
}

fn parse(input: &str) -> Vec<Note> {
    match separated_list0(tag("\n"), note)(input) {
        Ok((_remaining, ins)) => ins,
        Err(e) => panic!("{:?}", e),
    }
}

struct MonkeyBusiness<'a> {
    worries: Vec<VecDeque<u128>>,
    passes: Vec<usize>,
    rounds: usize,
    worry_manager: &'a dyn Fn(u128) -> u128,
    modulo: u128,
}

impl<'a> MonkeyBusiness<'a> {
    fn new(notes: &[Note], rounds: usize, worry_manager: &'a dyn Fn(u128) -> u128) -> Self {
        let mut worries: Vec<VecDeque<u128>> = vec![VecDeque::new(); notes.len()];
        let passes: Vec<usize> = vec![0; notes.len()];

        let modulo = notes.iter().map(|c| c.test.divisible_by).lcm().unwrap();

        // initial setup
        for i in 0..notes.len() {
            worries[i] = notes[i].items.clone().into();
        }

        Self {
            worries,
            passes,
            rounds,
            worry_manager,
            modulo,
        }
    }

    fn execute_command(&mut self, note: &Note) {
        let i = note.monkey_index;
        while let Some(worry_level) = self.worries[i].pop_front() {
            self.passes[i] += 1;
            let increased_worry_level = note.operation.updated_worry_level(worry_level);

            // Perform modulo arithmatic so that the numbers don't get too large
            let managed_worry_level =
                ((self.worry_manager)(increased_worry_level)).rem_euclid(self.modulo);

            let throw_to = note.test.throw_to(managed_worry_level);

            self.worries[throw_to].push_back(managed_worry_level)
        }
    }

    fn run(&mut self, notes: &[Note]) {
        for _round in 0..self.rounds {
            for note in notes {
                self.execute_command(note)
            }
        }
    }

    fn monkey_business(&self) -> usize {
        self.passes
            .iter()
            .sorted()
            .into_iter()
            .copied()
            .rev()
            .take(2)
            .product::<usize>()
    }
}

fn part1(notes: &[Note]) -> usize {
    let mut monkey_business = MonkeyBusiness::new(notes, 20, &|w| w / 3);
    monkey_business.run(notes);
    monkey_business.monkey_business()
}

fn part2(notes: &[Note]) -> usize {
    let mut monkey_business = MonkeyBusiness::new(notes, 10000, &|w| w);
    monkey_business.run(notes);
    monkey_business.monkey_business()
}

fn main() {
    let input = include_str!("../../data/day11.txt");
    let notes = parse(input);

    println!("Day 11 - Part 01: {}", part1(&notes));
    println!("Day 11 - Part 02: {}", part2(&notes));
}
