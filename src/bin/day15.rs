use aoc22::range::Merge;
use aoc22::{parsers::signed_decimal, range::Subsume};
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use std::ops::RangeInclusive;

type Location = (isize, isize);

#[derive(Debug)]
struct Pair {
    sensor: Location,
    beacon: Location,
}

impl Pair {
    fn manhattan(&self) -> usize {
        let (sensor_x, sensor_y) = self.sensor;
        let (beacon_x, beacon_y) = self.beacon;

        (beacon_x - sensor_x).abs() as usize + (beacon_y - sensor_y).abs() as usize
    }

    fn range_with_manhattan(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let m_distance = self.manhattan();
        if self.is_too_far(y) {
            None
        } else {
            let (sensor_x, sensor_y) = self.sensor;
            let remaining_m_distance = m_distance - ((y - sensor_y).abs() as usize);

            Some(
                (sensor_x - remaining_m_distance as isize)
                    ..=(sensor_x + remaining_m_distance as isize),
            )
            //
        }
    }

    fn is_too_far(&self, y: isize) -> bool {
        let (_sensor_x, sensor_y) = self.sensor;
        let m_distance = self.manhattan();

        (y - sensor_y).abs() as usize > m_distance
    }
}

// x=2, y=18
fn location(input: &str) -> IResult<&str, Location> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = signed_decimal(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, _) = tag("y=")(input)?;
    let (input, y) = signed_decimal(input)?;

    Ok((input, (x, y)))
}

fn sensor(input: &str) -> IResult<&str, Location> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, loc) = location(input)?;

    Ok((input, loc))
}

fn beacon(input: &str) -> IResult<&str, Location> {
    let (input, _) = tag("closest beacon is at ")(input)?;
    let (input, loc) = location(input)?;

    Ok((input, loc))
}

fn pair(input: &str) -> IResult<&str, Pair> {
    let (input, sensor) = sensor(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, beacon) = beacon(input)?;

    Ok((input, Pair { sensor, beacon }))
}

fn parse(input: &str) -> Vec<Pair> {
    match separated_list1(tag("\n"), pair)(input) {
        Ok((_remaining, res)) => res,
        Err(_) => panic!("invalid input"),
    }
}

fn merge_overlapping_intervals(
    intervals: Vec<RangeInclusive<isize>>,
) -> Vec<RangeInclusive<isize>> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut sorted_intervals = intervals;
    sorted_intervals.sort_unstable_by(|a, b| a.start().cmp(b.start()));
    let mut merged_intervals = vec![sorted_intervals[0].clone()];

    for interval in sorted_intervals[1..].iter() {
        let latest_interval = merged_intervals[merged_intervals.len() - 1].clone();
        if let Some(merged) = latest_interval.merge(interval) {
            merged_intervals.pop();
            merged_intervals.push(merged);
        } else {
            merged_intervals.push(interval.clone());
        }
    }

    merged_intervals
}

fn part1(pairs: &[Pair]) -> usize {
    let y = 2000000;

    let intervals = pairs
        .iter()
        .filter_map(|p| p.range_with_manhattan(y))
        .collect::<Vec<RangeInclusive<isize>>>();

    let range_without_beacons = merge_overlapping_intervals(intervals);

    match &range_without_beacons[..] {
        [range] => range.clone().count(),
        _ => panic!("Expected single range"),
    }
}

fn part2(pairs: &[Pair]) -> Option<isize> {
    let sensor_range_y = 0..=4000000;
    let sensor_range_x = 0..=4000000;

    for y in sensor_range_y {
        let ranges = pairs
            .iter()
            .filter_map(|p| p.range_with_manhattan(y))
            .collect::<Vec<RangeInclusive<isize>>>();
        let cant_have_beacon = merge_overlapping_intervals(ranges);

        match &cant_have_beacon[..] {
            [range] => {
                if range.subsumes(&sensor_range_x) {
                    continue;
                } else {
                    // The answer can only be 0 (the range starts at 1) or 4000000 (range ends at
                    // 3999999)
                    println!("{:?}", range);
                }
            }
            [range1, range2] => {
                assert!(range1.end() + 2 == *range2.start());
                let unique_points_x_coord = range1.end() + 1;
                let tuning_frequency = unique_points_x_coord * 4000000 + y;
                return Some(tuning_frequency);
            }
            _ => {
                panic!("Can't have more than 2 ranges to have unique position for faulty beacon");
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("../../data/day15.txt");
    let parsed = parse(input);
    println!("Day 15 - Part 01: {}", part1(&parsed));
    println!("Day 15 - Part 02: {}", part2(&parsed).unwrap());
}
