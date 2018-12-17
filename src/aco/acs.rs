use itertools::Itertools;

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
    /// Heuristic information based on the distance, calculated on initialization
    heuristic_info: FloatMatrix,
    /// Combined pheromone + heuristic information, recalculated every iteration
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
        let (heuristic_info, combined_info) = compute_combined_info(&data.distances,
                                                                   &pheromones,
                                                                   parameters);
        
        Self {
            iteration: 0,
            data,
            pheromones,
            heuristic_info,
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
    
    fn update_pheromones(&mut self, _: &AntResult, best_so_far: &AntResult) {
        global_update_pheromones(&mut self.pheromones, &mut self.heuristic_info, &mut self.combined_info, self.parameters, best_so_far);
    }
}

fn calculate_initial_values(nn_tour_length: usize, num_nodes: usize) -> f64 {
    1.0 / (num_nodes * nn_tour_length) as f64
}

fn global_update_pheromones(pheromones: &mut FloatMatrix, 
                            heuristic_info: &mut FloatMatrix,
                            combined_info: &mut FloatMatrix,
                            parameters: &AcoParameters,
                            best_so_far: &AntResult) {
    let d_tau = 1.0 / best_so_far.length as f64;
    let coefficient = 1.0 - parameters.evaporation_rate;
    for (&i,&j) in best_so_far.tour.iter().tuple_windows() {
        pheromones[i][j] = coefficient * pheromones[i][j] + parameters.evaporation_rate * d_tau;
        pheromones[j][i] = pheromones[i][j];
        combined_info[i][j] = super::total_value(pheromones[i][j],
                                                 heuristic_info[i][j],
                                                 parameters.alpha,
                                                 parameters.beta);
        combined_info[j][i] = combined_info[i][j];
    }   
}
