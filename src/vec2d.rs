use std::ops::{Index, IndexMut};
use std::rc::Rc;
use itertools::Itertools;

#[derive(Debug)]
pub struct Vec2D<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Vec2D<T> {
    pub fn from_rows(data: Vec<Vec<T>>) -> Self {
        assert_ne!(data.len(), 0);
        assert_ne!(data[0].len(), 0);

        let rows = data.len();
        let cols = data[0].len();

        Vec2D {
            data: data
                .into_iter()
                .map(|vec| vec.into_iter())
                .flatten()
                .collect_vec(),
            rows,
            cols,
        }
    }

    pub fn iter_mut_row_major(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows).map(|i| (i..(i + self.cols)).map(|i| &self.data[i]))
    }

    fn flatten_coords(&self, (x, y): (usize, usize)) -> Option<usize> {
        if x < self.rows && y < self.cols {
            Some(y * self.cols + x)
        } else {
            None
        }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        if let Some(index) = self.flatten_coords(index) {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        if let Some(index) = self.flatten_coords(index) {
            Some(&mut self.data[index])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}