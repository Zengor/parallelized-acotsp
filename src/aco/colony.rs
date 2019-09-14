use itertools::Itertools;

use super::ant::Ant;
use super::AcoParameters;
use crate::instance_data::InstanceData;
use crate::util::{self, FloatMatrix, IntegerMatrix};

pub trait Colony<'a> {
    fn new_iteration(&mut self);
    fn iteration(&self) -> usize;
    fn construct_solutions(&mut self) -> Ant;
    fn update_pheromones(&mut self, best_this_iter: &Ant, best_so_far: &Ant);
}

/// Generates and computes the heuristic info matrix and combined heuristic+pheromone matrix.
/// Returns a (heuristic, combined) tuple of FloatMatrixs.
/// Only to be used on initialization, use `recompute_combined_info` to fully update a combined
/// info matrix in-place.
pub fn compute_combined_info(
    distances: &IntegerMatrix,
    pheromones: &FloatMatrix,
    parameters: &AcoParameters,
) -> (FloatMatrix, FloatMatrix) {
    let mut heuristic_info = FloatMatrix::with_element(distances.width(), 0.0);
    let mut combined_info = FloatMatrix::with_element(distances.width(), 0.0);
    for i in 0..distances.width() {
        for j in 0..i {
            heuristic_info[(i, j)] = super::heuristic(distances, i, j);
            heuristic_info[(j, i)] = heuristic_info[(i, j)];
            combined_info[(i, j)] = super::total_value(
                pheromones[(i, j)],
                heuristic_info[(i, j)],
                parameters.alpha,
                parameters.beta,
            );
            combined_info[(j, i)] = combined_info[(i, j)];
        }
    }
    (heuristic_info, combined_info)
}

pub fn recompute_combined_info(
    combined_info: &mut FloatMatrix,
    pheromones: &FloatMatrix,
    heuristic_info: &FloatMatrix,
    parameters: &AcoParameters,
) {
    for i in 0..combined_info.width() {
        for j in 0..i {
            combined_info[(i, j)] = super::total_value(
                pheromones[(i, j)],
                heuristic_info[(i, j)],
                parameters.alpha,
                parameters.beta,
            );
            combined_info[(j, i)] = combined_info[(j, i)];
        }
    }
}

//fn generate_nn_list(data: &InstanceData, list_size: usize) -> Vec<Vec<usize>> {
//    let mut nn_list = Vec::with_capacity(data.size);
//    for i in 0..data.size {
//        let sorted = data.distances[i]
//            .iter()
//            .enumerate()
//            .sorted_by_key(|(_, &d)| d)
//            .take(list_size)
//            .map(|(c, _)| c)
//            .collect();
//        nn_list.push(sorted);
//    }
//    nn_list
//}
