// start location
// end location
// figure out elevation

use std::collections::{HashSet, VecDeque};

use aoc22::matrix::Matrix;

#[derive(Debug)]
enum Marker {
    Start,
    End,
}

type Location = (usize, usize);

fn parse(input: &str) -> Vec<Vec<(usize, Option<Marker>)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    'S' => (0, Some(Marker::Start)),
                    'E' => (26, Some(Marker::End)),
                    c => ((c as usize - 'a' as usize), None),
                })
                .collect::<Vec<(usize, Option<Marker>)>>()
        })
        .collect()
}

fn convert(input: Vec<Vec<(usize, Option<Marker>)>>) -> (Hill, Vec<Location>, Location) {
    // let mut heights = vec![vec![0; input[0].len()]; input.len()];
    let mut heights = Matrix::new((input.len(), input[0].len()));
    let mut starts = vec![];
    let mut end = None;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            heights[(i, j)] = input[i][j].0;
            match input[i][j].1 {
                Some(Marker::Start) => starts.push((i, j)),
                Some(Marker::End) => end = Some((i, j)),
                None => {}
            }
            if heights[(i, j)] == 0 {
                starts.push((i, j));
            }
        }
    }

    (Hill { heights }, starts, end.unwrap())
}

struct Hill {
    heights: Matrix<usize>,
}

impl Hill {
    fn neighbours(&self, loc: Location) -> impl Iterator<Item = Location> + '_ {
        let location_elevation = self.heights[loc];
        self.heights
            .neighbouring_indices(loc)
            .filter(move |(i, j)| self.heights[(*i, *j)] <= location_elevation + 1)
    }

    fn search(&self, start: Location, end: Location) -> usize {
        let mut queue: VecDeque<(Location, usize)> = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((position, steps)) = queue.pop_front() {
            if position == end {
                return steps;
            }

            let current_value = self.heights[position];

            for neighbour in self.neighbours(position) {
                let neighbour_value = self.heights[neighbour];
                if !visited.contains(&neighbour)
                    && (neighbour_value as isize - current_value as isize) <= 1
                {
                    visited.insert(neighbour);
                    queue.push_back((neighbour, steps + 1));
                }
            }
        }
        usize::MAX
    }
}

fn main() {
    let input = include_str!("../../data/day12.txt");
    let (hill, starts, end) = convert(parse(input));

    println!("{}", hill.search(starts[0], end));
    println!(
        "{}",
        starts.iter().map(|s| hill.search(*s, end)).min().unwrap()
    );
}
