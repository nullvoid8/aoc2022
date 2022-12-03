use cgmath::Point2;
use itertools::Itertools;

pub type Point = Point2<i32>;

#[derive(Debug)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn get<'a>(&'a self, p: Point) -> Option<&'a T> {
        if p.y < 0 || p.x < 0 {
            return None;
        }
        let (row, col) = (p.y as usize, p.x as usize);
        if row >= self.rows || col >= self.cols {
            return None;
        }
        self.data.get(col * self.cols + row)
    }

    pub fn get_mut<'a>(&'a mut self, p: Point) -> Option<&'a mut T> {
        if p.y < 0 || p.x < 0 {
            return None;
        }
        let (row, col) = (p.y as usize, p.x as usize);
        if row >= self.rows || col >= self.cols {
            return None;
        }
        self.data.get_mut(col * self.cols + row)
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        if self.cols == 0 {
            self.cols = row.len()
        }
        if row.len() != self.cols {
            panic!("column length mismatch");
        }

        self.data.extend(row);
        self.rows += 1;
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .map(|(row, col)| Point2 {
                y: row as i32,
                x: col as i32,
            })
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Grid {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }
}
