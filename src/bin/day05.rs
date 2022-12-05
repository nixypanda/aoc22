use aoc22::{math::transpose, parsers::decimal};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish,
};
use std::str::FromStr;

type Crate = String;
type CrateStack = Vec<Crate>;

#[derive(Debug)]
struct Command {
    crates: usize,
    from_stack: usize,
    to_stack: usize,
}

#[derive(Debug, Clone)]
struct Ship {
    stacks: Vec<CrateStack>,
    crate_mover: CrateMover,
}

#[derive(Debug, Clone)]
enum CrateMover {
    CM9000,
    CM9001,
}

impl Ship {
    fn move_crates(&mut self, command: &Command) {
        match self.crate_mover {
            CrateMover::CM9000 => {
                let from = command.from_stack - 1;
                let to = command.to_stack - 1;

                for _ in 0..command.crates {
                    let crat = self.stacks[from].pop();
                    self.stacks[to].push(crat.unwrap());
                }
            }
            CrateMover::CM9001 => {
                let from = command.from_stack - 1;
                let to = command.to_stack - 1;

                let final_length = self.stacks[from].len() - command.crates;
                let mut crates = self.stacks[from].split_off(final_length);
                self.stacks[to].append(&mut crates);
            }
        }
    }

    fn get_top_crates(&self) -> Vec<&Crate> {
        self.stacks.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn parse_command(input: &str) -> nom::IResult<&str, Command> {
    let (input, _) = tag("move ")(input)?;
    let (input, n) = decimal(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, f) = decimal(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, t) = decimal(input)?;

    let command = Command {
        crates: n,
        from_stack: f,
        to_stack: t,
    };

    Ok((input, command))
}

impl FromStr for Command {
    type Err = Error<Crate>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_command(s).finish() {
            Ok((_remaining, command)) => Ok(command),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn crat(input: &str) -> nom::IResult<&str, char> {
    delimited(tag("["), anychar, tag("]"))(input)
}

fn optional_crat(input: &str) -> nom::IResult<&str, char> {
    let empty_crate = map(tag("   "), |_| ' ');
    alt((empty_crate, crat))(input)
}

fn stack_frame(input: &str) -> nom::IResult<&str, Vec<char>> {
    separated_list1(tag(" "), optional_crat)(input)
}

fn stacks(input: &str) -> nom::IResult<&str, Vec<CrateStack>> {
    let (input, stacks) = separated_list1(tag("\n"), stack_frame)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = many1(one_of("123456789 "))(input)?;

    let result = transpose(stacks)
        .into_iter()
        .map(|line| {
            line.iter()
                .rev()
                .filter(|c| **c != ' ')
                .map(|c| c.to_string())
                .collect::<CrateStack>()
        })
        .collect::<Vec<CrateStack>>();

    Ok((input, result))
}

fn parse_stacks(f: &str) -> Vec<CrateStack> {
    match stacks(f) {
        Ok((_remaining, stack)) => stack,
        Err(e) => panic!("{:?}", e),
    }
}

fn parse(input: &str) -> (Vec<CrateStack>, Vec<Command>) {
    if let [stack, commands] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        let commands = commands
            .lines()
            .map(|command| command.parse().unwrap())
            .collect::<Vec<Command>>();

        (parse_stacks(stack), commands)
    } else {
        panic!()
    }
}

fn perform_commands(stack: &mut Ship, commands: &Vec<Command>) -> Crate {
    for command in commands {
        stack.move_crates(command)
    }
    stack.get_top_crates().iter().copied().join("")
}

fn main() {
    let input = include_str!("../../data/day05.txt");
    let (stacks, commands) = parse(input);

    let mut part1_ship = Ship {
        stacks: stacks.clone(),
        crate_mover: CrateMover::CM9000,
    };

    println!(
        "Day 05 - Part 01: {}",
        perform_commands(&mut part1_ship, &commands)
    );

    let mut part2_ship = Ship {
        stacks,
        crate_mover: CrateMover::CM9001,
    };
    println!(
        "Day 05 - Part 02: {}",
        perform_commands(&mut part2_ship, &commands)
    );
}
