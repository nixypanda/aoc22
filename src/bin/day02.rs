#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn hand_value(hand: Hand) -> usize {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn outcome_value(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    }
}

#[derive(Debug)]
struct Part1Round {
    opponent: Hand,
    player: Hand,
}

impl Part1Round {
    fn score(&self) -> usize {
        hand_value(self.player) + outcome_value(self.outcome())
    }

    fn outcome(&self) -> Outcome {
        match (self.player, self.opponent) {
            (Hand::Rock, Hand::Rock) => Outcome::Draw,
            (Hand::Rock, Hand::Paper) => Outcome::Loss,
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Paper) => Outcome::Draw,
            (Hand::Paper, Hand::Scissors) => Outcome::Loss,
            (Hand::Scissors, Hand::Rock) => Outcome::Loss,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
        }
    }
}

fn parse_part1_input(rounds: &str) -> Vec<Part1Round> {
    rounds
        .lines()
        .into_iter()
        .map(|round| parse_part1_round(round))
        .collect()
}

fn parse_part1_round(round: &str) -> Part1Round {
    match round.split(" ").collect::<Vec<&str>>()[..] {
        [p1, p2] => {
            return Part1Round {
                opponent: string_to_hand(p1),
                player: string_to_hand(p2),
            };
        }
        _ => {
            panic!("Invalid input")
        }
    }
}

fn string_to_hand(hand: &str) -> Hand {
    match hand {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn part1_total_score(rounds: &Vec<Part1Round>) -> usize {
    rounds.iter().map(|round| round.score()).sum()
}

////////////////////

struct Part2Round {
    opponent: Hand,
    required_outcome: Outcome,
}

fn parse_part2_input(rounds: &str) -> Vec<Part2Round> {
    rounds
        .lines()
        .into_iter()
        .map(|round| parse_part2_round(round))
        .collect()
}

fn parse_part2_round(round: &str) -> Part2Round {
    match round.split(" ").collect::<Vec<&str>>()[..] {
        [p1, p2] => {
            return Part2Round {
                opponent: string_to_hand(p1),
                required_outcome: string_to_outcome(p2),
            };
        }
        _ => {
            panic!("Invalid input")
        }
    }
}

fn string_to_outcome(outcome: &str) -> Outcome {
    match outcome {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid input"),
    }
}

impl Part2Round {
    fn score(&self) -> usize {
        hand_value(self.required_hand()) + outcome_value(self.required_outcome)
    }

    fn required_hand(&self) -> Hand {
        match (self.opponent, self.required_outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Rock, Outcome::Loss) => Hand::Scissors,
            (Hand::Rock, Outcome::Draw) => Hand::Rock,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Paper, Outcome::Loss) => Hand::Rock,
            (Hand::Paper, Outcome::Draw) => Hand::Paper,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Scissors, Outcome::Loss) => Hand::Paper,
            (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
        }
    }
}

fn part2_total_score(rounds: &Vec<Part2Round>) -> usize {
    rounds.iter().map(|round| round.score()).sum()
}

fn main() {
    let rounds = parse_part1_input(include_str!("../../data/day02.txt"));
    println!("Day 02 - Part 01: {}", part1_total_score(&rounds));
    let rounds = parse_part2_input(include_str!("../../data/day02.txt"));
    println!("Day 02 - Part 01: {}", part2_total_score(&rounds));
}
