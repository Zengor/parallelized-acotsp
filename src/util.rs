use indexmap::IndexSet;
use parking_lot::RwLock;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

pub type FloatMatrix = Matrix<f64>;
pub type IntegerMatrix = Matrix<u32>;
pub type FloatMatrixSync = Arc<Matrix<RwLock<f64>>>;

/// A very simple implementation of a matrix abstraction. It's
/// simply a Vec an associated 'width' that is used for indexing.
#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn row(&self, i: usize) -> &[T] {
        return &self.data[(i * self.width)..(i * self.width + self.width)];
    }

    /// Returns a Matrix with memory allocated for size*size elements.
    pub fn with_capacity(size: usize) -> Matrix<T> {
        Matrix {
            data: Vec::with_capacity(size * size),
            width: size,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
}

impl<T: Clone> Matrix<T> {
    /// Creates a size*size Matrix with the given initial element
    /// occupying all positions.
    pub fn with_element(size: usize, element: T) -> Matrix<T> {
        Matrix {
            data: vec![element; size * size],
            width: size,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let i = y * self.width + x;
        return &self.data[i];
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let i = y * self.width + x;
        return &mut self.data[i];
    }
}

trait ColonyInfoMatrix {
    fn unvisited_weights(&self, row: usize, excludes: &IndexSet<usize>) -> (Vec<usize>, Vec<f64>);
    fn filtered_row_max(&self, row: usize, excludes: &IndexSet<usize>) -> usize;
}

impl ColonyInfoMatrix for Matrix<f64> {
    fn unvisited_weights(&self, row: usize, excludes: &IndexSet<usize>) -> (Vec<usize>, Vec<f64>) {
        self.row(row)
            .iter()
            .enumerate()
            .filter(|(i, _)| !excludes.contains(i))
            .unzip()
    }
    fn filtered_row_max(&self, row: usize, excludes: &IndexSet<usize>) -> usize {
        let (i, _) = self
            .row(row)
            .iter()
            .enumerate()
            .filter(|(i, _)| !excludes.contains(i))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("failed comparison"))
            .expect("failed max_by");
        i
    }
}

impl ColonyInfoMatrix for Arc<Matrix<RwLock<f64>>> {
    fn unvisited_weights(&self, row: usize, excludes: &IndexSet<usize>) -> (Vec<usize>, Vec<f64>) {
        self.row(row)
            .iter()
            .enumerate()
            .filter(|(i, _)| !excludes.contains(i))
            .map(|(c, w)| (c, *w.read()))
            .unzip()
    }
    fn filtered_row_max(&self, row: usize, excludes: &IndexSet<usize>) -> usize {
        let (i, _) = self
            .row(row)
            .iter()
            .enumerate()
            .filter(|(i, _)| !excludes.contains(i))
            .max_by(|(_, a), (_, b)| {
                (a.read())
                    .partial_cmp(&b.read())
                    .expect("failed comparison")
            })
            .expect("failed max_by");
        i
    }
}

/// Creates a size*size Matrix meant to store pheromones with a given
/// initial value. The main diagonal will be set to f64::MAX so that
/// ants will not go repeatedly towards the same city without additional
/// checks.
pub fn generate_pheromone_matrix(size: usize, value: f64) -> FloatMatrix {
    let mut matrix = Matrix::with_element(size, value);
    for i in 0..size {
        matrix[(i, i)] = std::f64::MAX;
    }
    matrix
}

/// Wraps each element of a matrix in a `RwLock`, and then wraps
/// matrix in an `Arc`. Used with the parallelized version of ACS.
pub fn convert_to_sync(matrix: FloatMatrix) -> FloatMatrixSync {
    let width = matrix.width;
    let sync_vec = matrix.data.into_iter().map(|x| RwLock::new(x)).collect();

    Arc::new(Matrix {
        data: sync_vec,
        width,
    })
}

/// Calculates the value of a single tour (assumes first node is 0).
/// Was used while testing, left in just in case it is ever relevant again.
#[allow(dead_code)]
pub fn value_of_tour(distances: &IntegerMatrix, tour: &[usize]) -> u32 {
    use itertools::Itertools;
    let mut length = 0;
    for (&i, &j) in tour.iter().tuple_windows() {
        length += distances[(i, j)];
    }
    length += distances[(tour[tour.len() - 1], tour[0])];
    length
}

pub mod distance_funcs {
    pub fn euc_2d(i: (i32, i32), j: (i32, i32)) -> u32 {
        (((i.0 - j.0).pow(2) + (i.1 - j.1).pow(2)) as f64)
            .sqrt()
            .round() as u32
    }
}

#[cfg(test)]
pub mod test {
    use super::distance_funcs::*;
    use super::Matrix;

    #[test]
    pub fn euc_2d_1010_2020() {
        let x = (10, 10);
        let y = (20, 20);
        assert_eq!(euc_2d(x, y), 14);
    }

    #[test]
    pub fn matrix_test() {
        let size = 10;
        let mut matrix = Matrix::with_element(size, 0);
        for i in 0..size * size {
            matrix.data[i] = 0 + i;
        }
        assert_eq!(matrix.data[10], matrix[(0, 1)]);
        assert_eq!(matrix.data[1], matrix[(1, 0)]);
        assert_eq!(matrix.data[99], matrix[(9, 9)]);
        assert_eq!(matrix.row(0), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(matrix.row(3), &[30, 31, 32, 33, 34, 35, 36, 37, 38, 39]);
    }

}
