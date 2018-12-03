use crate::util::{self, FloatMatrix};
use crate::instance_data::InstanceData;

use super::ant;
use super::colony::{Colony, compute_combined_info};
use super::AcoParameters;
use super::AntResult;


pub struct ACSColony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: FloatMatrix,
    /// Combined pheromone + heuristic information
    combined_info: FloatMatrix,
    nn_list: Vec<Vec<usize>>,
    initial_trail: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for ACSColony<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> ACSColony<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data,0);
        let initial_trail = calculate_initial_values(nn_tour_length, data.size);
        let pheromones = util::generate_pheromone_matrix(data.size, initial_trail);
        let combined_info = compute_combined_info(&data.distances, &pheromones, parameters);
        
        Self {
            iteration: 0,
            data,
            pheromones,
            combined_info,
            nn_list: super::generate_nn_list(data),
            initial_trail,
            parameters,
        }
    }    
    
    fn new_iteration(&mut self) {
        self.iteration += 1
    }
    
    fn iteration(&self) -> usize {
        self.iteration
    }
    
    fn construct_solutions(&mut self) -> Vec<AntResult> {
        unimplemented!()
    }
    
    fn update_pheromones(&mut self, best_this_iter: &AntResult, best_so_far: &AntResult) {

    }
}

fn calculate_initial_values(nn_tour_length: usize, num_nodes: usize) -> f64 {
    1.0 / (num_nodes * nn_tour_length) as f64
}
