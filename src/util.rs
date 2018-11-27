pub type Matrix = Vec<Vec<f64>>;

pub fn generate_filled_matrix(size: usize, element: f64) -> Matrix {
    let mut out = Vec::with_capacity(size);
    for _ in 0..size {
        out.push(vec![element; size]);
    }
    out    
}

mod distance_funcs {

}
