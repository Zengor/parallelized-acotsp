use itertools::Itertools;
use rayon::prelude::*;

use super::{Ant, AcoParameters};
use super::ant::{self, mmas_ant};
use super::colony::{Colony, compute_combined_info, recompute_combined_info};
use crate::instance_data::InstanceData;
use crate::util::{self, FloatMatrix};

pub struct MMASColony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: FloatMatrix,
    /// Heuristic information based on the distance, calculated on initialization
    heuristic_info: FloatMatrix,
    /// Combined pheromone + heuristic information, recalculated every iteration
    combined_info: FloatMatrix,
    //nn_list: Vec<Vec<usize>>,    
    /// Maximum pheromone value for MMAS. This is calculated by the colony.
    pub trail_max: f64,
    /// Minimum pheromone value for MMAS. This is calculated by the colony.
    pub trail_min: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for MMASColony<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> MMASColony<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data, 0);
        let (trail_min, trail_max) = calculate_bounding_values(nn_tour_length,
                                                              data.size, 
                                                              parameters.evaporation_rate);
        let pheromones = util::generate_pheromone_matrix(data.size, trail_max);
        let (heuristic_info,combined_info) = compute_combined_info(&data.distances,
                                                                  &pheromones,
                                                                  parameters);
        
        Self {
            iteration: 0,
            data,
            pheromones,
            heuristic_info,
            combined_info,
            //nn_list: super::generate_nn_list(data),
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

    fn construct_solutions(&mut self) -> Vec<Ant> {
            //println!("new construction {}", self.iteration);
        let n_ants = self.parameters.num_ants;
        (0..n_ants).into_iter()
            .map(|_| mmas_ant(self.data, &self.combined_info))
            .collect()
    }
    
    fn update_pheromones(&mut self, best_this_iter: &Ant, best_so_far: &Ant) {
        let evap_rate = self.parameters.evaporation_rate;
        let (min, max) = calculate_bounding_values(best_so_far.length, self.data.size, evap_rate);
        self.trail_min = min;
        self.trail_max = max;
        evaporate(&mut self.pheromones, evap_rate, self.trail_min);        
        let ant_to_use = match self.iteration % 25 {
            0 => best_so_far,
            _ => best_this_iter,
        };
        global_update_pheromones(&mut self.pheromones, ant_to_use);
        recompute_combined_info(&mut self.combined_info, &self.heuristic_info, &self.pheromones, self.parameters);
    }
}

pub struct ParallelMMAS<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: FloatMatrix,
    /// Heuristic information based on the distance, calculated on initialization
    heuristic_info: FloatMatrix,
    /// Combined pheromone + heuristic information, recalculated every iteration
    combined_info: FloatMatrix,
    //nn_list: Vec<Vec<usize>>,    
    /// Maximum pheromone value for MMAS. This is calculated by the colony.
    pub trail_max: f64,
    /// Minimum pheromone value for MMAS. This is calculated by the colony.
    pub trail_min: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for ParallelMMAS<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> ParallelMMAS<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data, 0);
        let (trail_min, trail_max) = calculate_bounding_values(nn_tour_length,
                                                              data.size, 
                                                              parameters.evaporation_rate);
        let pheromones = util::generate_pheromone_matrix(data.size, trail_max);
        let (heuristic_info,combined_info) = compute_combined_info(&data.distances,
                                                                  &pheromones,
                                                                  parameters);
        
        Self {
            iteration: 0,
            data,
            pheromones,
            heuristic_info,
            combined_info,
            //nn_list: super::generate_nn_list(data),
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

    fn construct_solutions(&mut self) -> Vec<Ant> {
            //println!("new construction {}", self.iteration);
        let n_ants = self.parameters.num_ants;
        (0..n_ants).into_par_iter()
            .map(|_| mmas_ant(self.data, &self.combined_info))
            .collect()
    }
    
    fn update_pheromones(&mut self, best_this_iter: &Ant, best_so_far: &Ant) {
        let evap_rate = self.parameters.evaporation_rate;
        let (min, max) = calculate_bounding_values(best_so_far.length, self.data.size, evap_rate);
        self.trail_min = min;
        self.trail_max = max;
        evaporate(&mut self.pheromones, evap_rate, self.trail_min);        
        let ant_to_use = match self.iteration % 25 {
            0 => best_so_far,
            _ => best_this_iter,
        };
        global_update_pheromones(&mut self.pheromones, ant_to_use);
        recompute_combined_info(&mut self.combined_info, &self.heuristic_info, &self.pheromones, self.parameters);
    }
}

/// Calculates trail_min and trail_max for MMAS given best tour length. trail_max is to be used as initial pheormone value.
/// 
/// Returns tuple (trail_min, trail_max)
fn calculate_bounding_values(tour_length: u32,
                            num_nodes: usize,
                            evaporation_rate: f64) -> (f64, f64) {
    let trail_max = 1.0 / (evaporation_rate * tour_length as f64);
    let trail_min = trail_max / (2.0 * num_nodes as f64);
    (trail_min, trail_max)
}

fn evaporate(pheromones: &mut FloatMatrix,
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

pub fn global_update_pheromones(pheromones: &mut FloatMatrix, ant: &Ant) {
    let d_tau = 1.0 / (ant.length as f64);
    for (&i,&j) in ant.tour.iter().tuple_windows() {
        pheromones[i][j] += d_tau;
        pheromones[j][i] =  pheromones[i][j];
    }   
}