pub type FloatMatrix = Vec<Vec<f64>>;
pub type IntegerMatrix = Vec<Vec<u32>>;

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

/// Calculates the value of a single tour (assumes first node is 0), used while testing, left in just in case.
#[allow(dead_code)]
pub fn value_of_tour(distances: &IntegerMatrix, tour: &[usize]) -> u32 {
    use itertools::Itertools;
    let mut length = 0;
    for (&i, &j) in tour.iter().tuple_windows() {
        length += distances[i][j];
    }
    length+= distances[tour[tour.len()-1]][tour[0]];
    length
}

pub mod distance_funcs {
    pub fn euc_2d(i: (i32, i32), j: (i32, i32)) -> u32 {
        (((i.0 - j.0).pow(2) + (i.1 - j.1).pow(2)) as f64).sqrt().round() as u32
    }
}

#[cfg(test)]
pub mod test {
    use super::distance_funcs::*;

    #[test]
    pub fn euc_2d_1010_2020() {
        let x = (10,10);
        let y = (20,20);
        assert_eq!(euc_2d(x,y), 14);
    }

}