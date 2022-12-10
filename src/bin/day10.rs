use aoc22::parsers::signed_decimal;
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0, IResult};

const INSTRUCTIONS_TO_SKIP: usize = 19;
const CRT_SIZE: usize = 40;

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    NoOp,
}

fn no_op(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::NoOp)(input)
}

fn add_x(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, v) = signed_decimal(input)?;

    Ok((input, Instruction::AddX(v)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(tag("\n"), alt((add_x, no_op)))(input)
}

fn parse(input: &str) -> Vec<Instruction> {
    match instructions(input) {
        Ok((_remaining, ins)) => ins,
        Err(e) => panic!("{:?}", e),
    }
}

struct Cpu {
    register: usize,
    sprite_positions: Vec<usize>,
}

impl Cpu {
    fn perform_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(new_val) => {
                self.sprite_positions.push(self.register);
                self.register = (self.register as isize + new_val) as usize;
            }
            Instruction::NoOp => {}
        };
        self.sprite_positions.push(self.register);
    }

    fn steps(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (1..).zip(self.sprite_positions.iter().copied())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            register: 1,
            sprite_positions: vec![1],
        }
    }
}

fn perform_instructions(cpu: &mut Cpu, instructions: Vec<Instruction>) {
    for instruction in instructions {
        cpu.perform_instruction(instruction);
    }
}

fn part1(cpu: &Cpu) -> usize {
    cpu.steps()
        .skip(INSTRUCTIONS_TO_SKIP)
        .step_by(CRT_SIZE)
        .map(|(idx, register)| idx * register)
        .sum::<usize>()
}

fn crt_position_at_cycle(cycle: usize) -> usize {
    (cycle - 1) % CRT_SIZE
}

fn crt_sprite_overlaps_with_current_pixel(sprite_center: usize, current_pixel: usize) -> bool {
    current_pixel as isize >= (sprite_center as isize - 1)
        && current_pixel as isize <= (sprite_center as isize + 1)
}

fn crt_draw(cycle: usize, sprite_center: usize) -> char {
    let current_pixel = crt_position_at_cycle(cycle);
    if crt_sprite_overlaps_with_current_pixel(sprite_center, current_pixel) {
        '#'
    } else {
        '.'
    }
}

fn part2(cpu: &Cpu) -> String {
    cpu.steps()
        .map(|(c, sp)| crt_draw(c, sp))
        .chunks(CRT_SIZE)
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .join("\n")
}

fn main() {
    let ins = parse(include_str!("../../data/day10.txt"));
    let mut cpu = Cpu::default();
    perform_instructions(&mut cpu, ins);
    println!("Day 10 - Part 01: {}", part1(&cpu));
    println!("Day 10 - Part 02: \n{}", part2(&cpu));
}
