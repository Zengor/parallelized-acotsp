pub type Matrix = Vec<Vec<f64>>;

pub fn generate_empty_matrix(size: usize) -> Matrix {
    let mut out = Vec::with_capacity(size);
    for _ in 0..size {
        out.push(vec![0.0; size]);
    }
    out    
}

mod distance_funcs {

}
