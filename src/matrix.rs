use std::ops;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

type Index = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct InnerIndex(usize, usize);

impl InnerIndex {
    fn row(&self) -> usize {
        self.0
    }
    fn col(&self) -> usize {
        self.1
    }
}

impl From<InnerIndex> for (usize, usize) {
    fn from(idx: InnerIndex) -> Self {
        (idx.row(), idx.col())
    }
}
impl From<(usize, usize)> for InnerIndex {
    fn from((i, j): (usize, usize)) -> Self {
        InnerIndex(i, j)
    }
}

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn number_of_rows(&self) -> usize {
        self.rows
    }

    pub fn number_of_cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, (i, j): Index) -> Option<&T> {
        if i <= self.rows && j <= self.cols {
            Some(&self.data[i][j])
        } else {
            None
        }
    }

    pub fn elements_to_right(&self, idx: Index) -> ElementsInDirection<T> {
        self.elements_in_direction(idx, Direction::Right)
    }

    pub fn elements_to_left(&self, idx: Index) -> ElementsInDirection<T> {
        self.elements_in_direction(idx, Direction::Left)
    }

    pub fn elements_to_top(&self, idx: Index) -> ElementsInDirection<T> {
        self.elements_in_direction(idx, Direction::Top)
    }

    pub fn elements_to_bottom(&self, idx: Index) -> ElementsInDirection<T> {
        self.elements_in_direction(idx, Direction::Bottom)
    }

    fn next_in_direction(
        &self,
        current_index: InnerIndex,
        direction: Direction,
    ) -> Option<InnerIndex> {
        match direction {
            Direction::Left => {
                if current_index.col() == 0 {
                    None
                } else {
                    Some(InnerIndex(current_index.row(), current_index.col() - 1))
                }
            }
            Direction::Right => {
                if current_index.col() >= self.number_of_cols() - 1 {
                    None
                } else {
                    Some(InnerIndex(current_index.row(), current_index.col() + 1))
                }
            }
            Direction::Top => {
                if current_index.row() == 0 {
                    None
                } else {
                    Some(InnerIndex(current_index.row() - 1, current_index.col()))
                }
            }
            Direction::Bottom => {
                if current_index.row() >= self.number_of_rows() - 1 {
                    None
                } else {
                    Some(InnerIndex(current_index.row() + 1, current_index.col()))
                }
            }
        }
    }

    pub fn elements_in_direction(
        &self,
        idx: Index,
        direction: Direction,
    ) -> ElementsInDirection<T> {
        let idx = idx.into();
        let current_index = self.next_in_direction(idx, direction);

        ElementsInDirection {
            data: self,
            current_index,
            direction,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();

        Matrix { data, rows, cols }
    }
}

impl<T> ops::Index<Index> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> ops::IndexMut<Index> for Matrix<T> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

pub struct ElementsInDirection<'a, T> {
    data: &'a Matrix<T>,
    // start_index: Index,
    current_index: Option<InnerIndex>,
    direction: Direction,
}

impl<'a, T> Iterator for ElementsInDirection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_index) = self.current_index {
            if let Some(element) = self.data.get(current_index.into()) {
                self.current_index = self.data.next_in_direction(current_index, self.direction);
                Some(element)
            } else {
                None
            }
        } else {
            None
        }
    }
}
