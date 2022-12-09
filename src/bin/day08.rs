#![allow(clippy::needless_range_loop)]

use aoc22::matrix::{Direction, Matrix};

fn parse(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .into_iter()
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

struct Forest {
    trees: Matrix<usize>,
}

impl Forest {
    fn number_of_rows(&self) -> usize {
        self.trees.number_of_rows()
    }

    fn number_of_cols(&self) -> usize {
        self.trees.number_of_cols()
    }

    fn visible_in_direction(
        &self,
        (i, j): (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = &usize> {
        let current = self.trees[(i, j)];
        self.trees
            .elements_in_direction((i, j), direction)
            .take_while(move |e| **e < current)
            .chain(
                self.trees
                    .elements_in_direction((i, j), direction)
                    .find(|x| **x >= current),
            )
    }

    fn can_see_the_edge_in_direction(&self, (i, j): (usize, usize), direction: Direction) -> bool {
        let current = self.trees[(i, j)];
        let count = self
            .visible_in_direction((i, j), direction)
            .take_while(|e| **e < current)
            .into_iter()
            .count();
        let expected_count = match direction {
            Direction::Left => j,
            Direction::Right => self.number_of_cols() - j - 1,
            Direction::Top => i,
            Direction::Bottom => self.number_of_rows() - i - 1,
        };

        count == expected_count
    }

    fn can_see_the_edge(&self, (i, j): (usize, usize)) -> bool {
        self.can_see_the_edge_in_direction((i, j), Direction::Left)
            || self.can_see_the_edge_in_direction((i, j), Direction::Right)
            || self.can_see_the_edge_in_direction((i, j), Direction::Top)
            || self.can_see_the_edge_in_direction((i, j), Direction::Bottom)
    }

    fn visible_to_left(&self, idx: (usize, usize)) -> impl IntoIterator<Item = &usize> {
        self.visible_in_direction(idx, Direction::Left)
    }

    fn visible_to_right(&self, idx: (usize, usize)) -> impl IntoIterator<Item = &usize> {
        self.visible_in_direction(idx, Direction::Right)
    }

    fn visible_to_top(&self, idx: (usize, usize)) -> impl IntoIterator<Item = &usize> {
        self.visible_in_direction(idx, Direction::Top)
    }

    fn visible_to_bottom(&self, idx: (usize, usize)) -> impl IntoIterator<Item = &usize> {
        self.visible_in_direction(idx, Direction::Bottom)
    }
}

fn secenic_score(forest: &Forest) -> Vec<Vec<usize>> {
    let rows = forest.number_of_rows();
    let columns = forest.number_of_cols();

    let mut secenic_score = vec![vec![0; columns]; rows];

    for i in 0..rows {
        for j in 0..columns {
            let left = forest.visible_to_left((i, j)).into_iter().count();
            let right = forest.visible_to_right((i, j)).into_iter().count();
            let top = forest.visible_to_top((i, j)).into_iter().count();
            let bottom = forest.visible_to_bottom((i, j)).into_iter().count();

            secenic_score[i][j] = left * right * top * bottom;
        }
    }

    secenic_score
}

fn visibility_score(forest: &Forest) -> Vec<Vec<bool>> {
    let rows = forest.number_of_rows();
    let columns = forest.number_of_cols();

    let mut visibile = vec![vec![false; columns]; rows];

    for i in 0..rows {
        for j in 0..columns {
            visibile[i][j] = forest.can_see_the_edge((i, j));
        }
    }

    visibile
}

fn part1(forest: &Forest) -> usize {
    let visibility_scores = visibility_score(forest);
    visibility_scores
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .filter(|&&b| b)
        .count()
}

fn part2(forest: &Forest) -> usize {
    let secenic_scores = secenic_score(forest);
    *secenic_scores
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .max()
        .unwrap()
}

fn main() {
    let data = include_str!("../../data/day08.txt");
    let forest = Forest {
        trees: parse(data).into(),
    };

    println!("{:?}", part1(&forest));
    println!("{:?}", part2(&forest));
}
