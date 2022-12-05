use itertools::Itertools;

type Calorie = usize;
type CaloriesCarriedByElf = Vec<Calorie>;

fn parse_calories_carried_by_elves(calorie_list: &str) -> Vec<CaloriesCarriedByElf> {
    calorie_list
        .split("\n\n")
        .map(|calories_carried_by_elf| {
            calories_carried_by_elf
                .lines()
                .map(|calories_in_meal| calories_in_meal.parse::<Calorie>().unwrap())
                .collect::<CaloriesCarriedByElf>()
        })
        .collect()
}

fn part1_most_calories_carried(calorie_list: &[CaloriesCarriedByElf]) -> Calorie {
    calorie_list
        .iter()
        .map(|calorie_carried_by_elf| calorie_carried_by_elf.iter().sum::<Calorie>())
        .max()
        .unwrap()
}

fn part2_most_calories_carried_by_3_elves(calorie_list: &[CaloriesCarriedByElf]) -> Calorie {
    calorie_list
        .iter()
        .map(|calorie_carried_by_elf| calorie_carried_by_elf.iter().sum::<Calorie>())
        .sorted()
        .rev()
        .take(3)
        .sum::<Calorie>()
}

fn main() {
    let calories_carried_by_elves = include_str!("../../data/day01.txt");
    let parsed_input = parse_calories_carried_by_elves(calories_carried_by_elves);

    println!(
        "Day 01 - Part 01: {}",
        part1_most_calories_carried(&parsed_input)
    );
    println!(
        "Day 01 - Part 01: {}",
        part2_most_calories_carried_by_3_elves(&parsed_input)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    fn input() -> Vec<CaloriesCarriedByElf> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn parsing_works() {
        assert_eq!(input(), parse_calories_carried_by_elves(TEST_INPUT))
    }

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1_most_calories_carried(&input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2_most_calories_carried_by_3_elves(&input()));
    }
}
