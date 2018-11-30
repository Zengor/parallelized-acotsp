use itertools::Itertools;

use super::{AntResult, AcoParameters};
use super::ant::{self, mmas_ant};
use crate::instance_data::InstanceData;
use crate::util::{self, PheromoneMatrix};

pub struct MMASColony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: PheromoneMatrix,
    /// Combined pheromone + heuristic information
    combined_info: PheromoneMatrix,
    nn_list: Vec<Vec<usize>>,    
    /// Maximum pheromone value for MMAS. This is calculated by the colony.
    pub trail_max: f64,
    /// Minimum pheromone value for MMAS. This is calculated by the colony.
    pub trail_min: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for MMASColony<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> MMASColony<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data, 0);
    
        let (trail_min, trail_max) = calculate_initial_values(nn_tour_length,
                                                              data.size, 
                                                              parameters);
        let pheromones = util::generate_pheromone_matrix(data.size, trail_max);
        let combined_info = compute_combined_info(&data.distances, &pheromones, parameters);
        
        Self {
            iteration: 0,
            data,
            pheromones,
            combined_info,
            nn_list: super::generate_nn_list(data),
            trail_max,
            trail_min,
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
        let n_ants = self.parameters.num_ants;
        (0..n_ants).into_iter()
            .map(|_| mmas_ant(self.data, &self.pheromones, self.parameters))
            .collect()
    }
    
    fn update_pheromones(&mut self, best_this_iter: &AntResult, best_so_far: &AntResult) {
        evaporate(&mut self.pheromones, self.parameters.evaporation_rate, self.trail_min);        
        let ant_to_use = match self.iteration % 25 {
            0 => best_so_far,
            _ => best_this_iter,
        };
        ant::global_update_pheromones(&mut self.pheromones, ant_to_use);
    }
}

/// Calculates initial pheromone trails, as well as trail_max and trail_min for MMAS
fn calculate_initial_values(nn_tour_length: usize,
                            num_nodes: usize,
                            parameters: &AcoParameters) -> (f64, f64) {
    let trail_max = 1.0 / (parameters.evaporation_rate * nn_tour_length as f64);
    let trail_min = trail_max / (2.0 * num_nodes as f64);
    (trail_min, trail_max)
}

fn evaporate(pheromones: &mut PheromoneMatrix,
             evap_rate: f64,
             trail_min: f64) {
    for i in 0..pheromones.len() {
        for j in 0..i {
            let mut new_pheromone = (1.0 - evap_rate) * pheromones[i][j];
            if new_pheromone < trail_min {
                new_pheromone = trail_min;
            }
            pheromones[i][j] = new_pheromone;
            pheromones[j][i] = new_pheromone;
        }
    }
}



