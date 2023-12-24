use std::{
    fmt::Display,
    ops::{Index, IndexMut, Range},
};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<char>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<char>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }

    pub fn as_coordinates(&self, index: usize) -> (usize, usize) {
        let row = index / self.cols;
        let col = index % self.cols;

        (row, col)
    }

    pub fn as_index(&self, coordinates: (usize, usize)) -> usize {
        coordinates.0 * self.cols + coordinates.1 % self.cols
    }

    pub fn insert(&mut self, index: usize, element: char) {
        self.data.insert(index, element)
    }

    pub fn insert_row(&mut self, row_index: usize, elements: Vec<char>) {
        assert_eq!(self.cols, elements.len());
        assert!(row_index < self.rows);
        for e in elements.into_iter() {
            self.insert(row_index * self.cols, e);
        }
        self.rows += 1
    }

    pub fn replace_row(&mut self, row_index: usize, elements: Vec<char>) {
        assert_eq!(self.cols, elements.len());
        assert!(row_index < self.rows);
        for (i, e) in elements.into_iter().enumerate() {
            self.data[row_index * self.cols + i] = e;
        }
    }

    pub fn insert_col(&mut self, col_index: usize, elements: Vec<char>) {
        assert_eq!(self.rows, elements.len());
        assert!(col_index < self.cols);

        for (i, e) in elements.into_iter().enumerate() {
            self.insert(col_index + i * self.cols + i, e);
        }
        self.cols += 1
    }

    pub fn replace_col(&mut self, col_index: usize, elements: Vec<char>) {
        assert_eq!(self.rows, elements.len());
        assert!(col_index < self.cols);

        for (i, e) in elements.into_iter().enumerate() {
            self.data[col_index + i * self.cols] = e;
        }
    }

    pub fn get_row(&self, row_index: usize) -> Vec<char> {
        assert!(row_index < self.rows);

        (self.data[row_index * self.cols..row_index * self.cols + self.cols]).to_vec()
    }

    pub fn get_rows(&self, row_indices: Range<usize>) -> Vec<Vec<char>> {
        let mut rows: Vec<Vec<char>> = vec![];

        for i in row_indices {
            rows.push(self.get_row(i));
        }

        rows
    }

    pub fn get_col(&self, col_index: usize) -> Vec<char> {
        assert!(col_index < self.cols);

        let mut col: Vec<char> = vec![];

        for i in 0..self.rows {
            col.push(self.data[col_index + i * self.cols])
        }

        col
    }

    pub fn get_col_mut(&self, col_index: usize) -> Vec<char> {
        assert!(col_index < self.cols);

        let mut col: Vec<char> = vec![];

        for i in 0..self.rows {
            col.push(self.data[col_index + i * self.cols])
        }

        col
    }

    pub fn get_cols(&self, col_indices: Range<usize>) -> Vec<Vec<char>> {
        let mut cols: Vec<Vec<char>> = vec![];

        for i in col_indices {
            cols.push(self.get_col(i));
        }

        cols
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.data.iter()
    }

    pub fn clone(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }

    pub fn rotate_right(&mut self) {
        let mut rotated: Vec<char> = vec![];

        for i in 0..self.cols {
            for j in 0..self.rows {
                rotated.push(self.data[(self.rows - 1 - j) * self.cols + i]);
            }
        }

        self.data = rotated;

        std::mem::swap(&mut self.rows, &mut self.cols);
    }

    pub fn rotate_left(&mut self) {
        let mut rotated: Vec<char> = vec![];

        for i in 0..self.cols {
            for j in 0..self.rows {
                rotated.push(self.data[j * self.cols + (self.cols - 1 - i)]);
            }
        }

        self.data = rotated;

        std::mem::swap(&mut self.rows, &mut self.cols);
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            let mut row_string = String::new();
            for c in self.data.iter().skip(i * self.cols).take(self.cols) {
                row_string.push(*c);
            }
            print!("{}", row_string);
            println!();
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.data.iter().collect();
        write!(f, "{}", s)
    }
}
