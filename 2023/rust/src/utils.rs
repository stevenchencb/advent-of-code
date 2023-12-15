use regex::{Captures, Regex};
use std::{
    fs::read_to_string,
    ops::{Index, Range},
};

pub fn get_file_lines(file: &str) -> Vec<String> {
    return read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

pub fn extract<'a>(s: &'a str, regex: &'a str) -> Vec<&'a str> {
    let regex = Regex::new(regex).unwrap();

    return regex.find_iter(s).map(|m| m.as_str()).collect();
}

pub fn extract_named<'a>(s: &'a str, regex: &'a str, group_name: &'a str) -> String {
    let regex = Regex::new(regex).unwrap();
    let extracted = regex.captures(s);

    match extracted {
        Some(extracted) => extracted[group_name].to_string(),
        None => String::new(),
    }
}

pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T: std::clone::Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
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

    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element)
    }

    pub fn insert_row(&mut self, row_index: usize, elements: Vec<T>) {
        assert_eq!(self.cols, elements.len());
        assert!(row_index < self.rows);
        for e in elements.into_iter() {
            self.insert(row_index * self.cols, e);
        }
        self.rows += 1
    }

    pub fn insert_col(&mut self, col_index: usize, elements: Vec<T>) {
        assert_eq!(self.rows, elements.len());
        assert!(col_index < self.cols);

        for (i, e) in elements.into_iter().enumerate() {
            self.insert(col_index + i * self.cols + i, e);
        }
        self.cols += 1
    }

    pub fn get_row(&self, row_index: usize) -> Vec<T> {
        assert!(row_index < self.rows);

        (self.data[row_index * self.cols..row_index * self.cols + self.cols]).to_vec()
    }

    pub fn get_rows(&self, row_indices: Range<usize>) -> Vec<Vec<T>> {
        let mut rows: Vec<Vec<T>> = vec![];

        for i in row_indices {
            rows.push(self.get_row(i));
        }

        rows
    }

    pub fn get_col(&self, col_index: usize) -> Vec<T> {
        assert!(col_index < self.cols);

        let mut col: Vec<T> = vec![];

        for i in 0..self.rows {
            col.push(self.data[col_index + i * self.cols].clone())
        }

        col
    }

    pub fn get_cols(&self, col_indices: Range<usize>) -> Vec<Vec<T>> {
        let mut cols: Vec<Vec<T>> = vec![];

        for i in col_indices {
            cols.push(self.get_col(i));
        }

        cols
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn clone(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}
