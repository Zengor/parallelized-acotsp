use parking_lot::RwLock;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

pub type FloatMatrix = Vec<Vec<f64>>;
pub type IntegerMatrix = Vec<Vec<u32>>;
pub type FloatMatrixSync = Arc<Vec<Vec<Arc<RwLock<f64>>>>>;

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: usize,
}

impl<T: Clone> Matrix<T> {
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

pub fn generate_pheromone_matrix(size: usize, value: f64) -> FloatMatrix {
    let mut matrix = generate_filled_matrix(size, value);
    for i in 0..size {
        matrix[i][i] = std::f64::MAX;
    }
    matrix
}

pub fn generate_filled_matrix<T: Copy>(size: usize, element: T) -> Vec<Vec<T>> {
    let mut out = Vec::with_capacity(size);
    for _ in 0..size {
        out.push(vec![element; size]);
    }
    out
}

pub fn convert_to_sync(matrix: FloatMatrix) -> FloatMatrixSync {
    let mut outer = Vec::with_capacity(matrix.len());
    for inner in matrix.into_iter() {
        outer.push(
            inner
                .into_iter()
                .map(|x| Arc::new(RwLock::new(x)))
                .collect(),
        );
    }
    Arc::new(outer)
}

/// Calculates the value of a single tour (assumes first node is 0).
/// Was used while testing, left in just in case it is ever relevant again.
#[allow(dead_code)]
pub fn value_of_tour(distances: &IntegerMatrix, tour: &[usize]) -> u32 {
    use itertools::Itertools;
    let mut length = 0;
    for (&i, &j) in tour.iter().tuple_windows() {
        length += distances[i][j];
    }
    length += distances[tour[tour.len() - 1]][tour[0]];
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
    pub fn matrix_text() {
        let size = 10;
        let mut matrix = Matrix::with_element(size, 0.0);
        for i in 0..size * size {
            matrix.data[i] = 0.0 + i as f32;
        }
        assert_eq!(matrix.data[10], matrix[(0, 1)]);
        assert_eq!(matrix.data[1], matrix[(1, 0)]);
        assert_eq!(matrix.data[99], matrix[(9, 9)])
    }

}
