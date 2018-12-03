use super::AcoParameters;
use super::ant::AntResult;
use crate::instance_data::InstanceData;
use crate::util::{self, IntegerMatrix, FloatMatrix};

pub trait Colony<'a> {    
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> Self;
    fn new_iteration(&mut self);
    fn iteration(&self) -> usize;
    fn construct_solutions(&mut self) -> Vec<AntResult>;
    fn update_pheromones(&mut self, best_this_iter: &AntResult, best_so_far: &AntResult);
}

pub fn compute_combined_info(distances: &IntegerMatrix,
                             pheromones: &FloatMatrix,
                             parameters: &AcoParameters) -> (FloatMatrix, FloatMatrix) {
    let mut heuristic_info = util::generate_filled_matrix(distances.len(), 0.0);
    let mut combined_info = util::generate_filled_matrix(distances.len(), 0.0);
    for i in 0..distances.len() {
        for j in 0..i {
            heuristic_info[i][j] = super::heuristic(distances, i, j);
            heuristic_info[j][i] = heuristic_info[i][j];
            combined_info[i][j] = super::total_value(pheromones[i][j],
                                                     heuristic_info[i][j],
                                                     parameters);
            combined_info[j][i] = combined_info[i][j];
        }
    }
    (heuristic_info, combined_info)
}
