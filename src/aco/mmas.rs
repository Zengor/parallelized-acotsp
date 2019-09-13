use itertools::Itertools;
use rayon::prelude::*;

use super::ant::{self, mmas_ant};
use super::colony::{compute_combined_info, recompute_combined_info, Colony};
use super::{AcoParameters, Ant};
use crate::instance_data::InstanceData;
use crate::util::{self, FloatMatrix};

pub struct MMASColony<'a> {
    iteration: usize,
    parallel: bool,
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
    restart_ant: Option<Ant>,
    restart_iter: usize,
}

impl<'a> Colony<'a> for MMASColony<'a> {
    fn new_iteration(&mut self) {
        self.iteration += 1;
        if self.iteration - self.restart_iter >= 150 {
            println!("restart");
            self.restart_ant = None;
            self.reinitialize_trails();
            recompute_combined_info(
                &mut self.combined_info,
                &self.pheromones,
                &self.heuristic_info,
                self.parameters,
            );
            self.restart_iter = self.iteration;
        }
    }

    fn iteration(&self) -> usize {
        self.iteration
    }

    fn construct_solutions(&mut self) -> Ant {
        //println!("new construction {}", self.iteration);
        let range = 0..self.parameters.num_ants;
        if self.parallel {
            range
                .into_par_iter()
                .map(|_| mmas_ant(self.data, &self.combined_info))
                .min_by_key(|a| a.length)
                .unwrap()
        } else {
            range
                .into_iter()
                .map(|_| mmas_ant(self.data, &self.combined_info))
                .min_by_key(|a| a.length)
                .unwrap()
        }
    }

    fn update_pheromones(&mut self, best_this_iter: &Ant, best_so_far: &Ant) {
        if self.restart_ant.is_none()
            || best_this_iter.length < self.restart_ant.as_ref().unwrap().length
        {
            self.restart_ant = Some(best_this_iter.clone());
            self.restart_iter = self.iteration;
        }
        let evap_rate = self.parameters.evaporation_rate;
        let (min, max) = calculate_bounding_values(best_so_far.length, self.data.size, evap_rate);
        self.trail_min = min;
        self.trail_max = max;
        evaporate(&mut self.pheromones, evap_rate);
        let ant_to_use = match self.iteration % 25 {
            0 => &self.restart_ant.as_ref().unwrap_or(best_so_far),
            _ => best_this_iter,
        };
        global_update_pheromones(&mut self.pheromones, ant_to_use);
        self.check_trail_limits();
        recompute_combined_info(
            &mut self.combined_info,
            &self.pheromones,
            &self.heuristic_info,
            self.parameters,
        );
    }
}

impl<'a> MMASColony<'a> {
    pub fn initialize_colony(
        data: &'a InstanceData,
        parameters: &'a AcoParameters,
        parallel: bool,
    ) -> MMASColony<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data, 0);
        let (trail_min, trail_max) =
            calculate_bounding_values(nn_tour_length, data.size, parameters.evaporation_rate);
        let pheromones = util::generate_pheromone_matrix(data.size, trail_max);
        let (heuristic_info, combined_info) =
            compute_combined_info(&data.distances, &pheromones, parameters);

        Self {
            iteration: 0,
            parallel,
            data,
            pheromones,
            heuristic_info,
            combined_info,
            //nn_list: super::generate_nn_list(data),
            trail_max,
            trail_min,
            parameters,
            restart_ant: None,
            restart_iter: 1,
        }
    }
    
    fn check_trail_limits(&mut self) {
        for i in 0..self.pheromones.width() {
            for j in 0..i {
                if self.pheromones[(i, j)] < self.trail_min {
                    self.pheromones[(i, j)] = self.trail_min;
                    self.pheromones[(j, i)] = self.trail_min;
                }
                if self.pheromones[(i, j)] > self.trail_max {
                    self.pheromones[(i, j)] = self.trail_max;
                    self.pheromones[(j, i)] = self.trail_max;
                }
            }
        }
    }

    fn reinitialize_trails(&mut self) {
        for i in 0..self.pheromones.width() {
            for j in 0..i {
                self.pheromones[(i, j)] = self.trail_max;
                self.pheromones[(j, i)] = self.trail_max;
            }
        }
    }
}
/// Calculates trail_min and trail_max for MMAS given best tour length. trail_max is to be used as initial pheormone value.
///
/// Returns tuple (trail_min, trail_max)
fn calculate_bounding_values(
    tour_length: u32,
    num_nodes: usize,
    evaporation_rate: f64,
) -> (f64, f64) {
    let trail_max = 1.0 / (evaporation_rate * tour_length as f64);
    let trail_min = trail_max / (2.0 * num_nodes as f64);
    (trail_min, trail_max)
}

fn evaporate(pheromones: &mut FloatMatrix, evap_rate: f64) {
    for i in 0..pheromones.width() {
        for j in 0..i {
            pheromones[(i, j)] = (1.0 - evap_rate) * pheromones[(i, j)];
            pheromones[(j, i)] = pheromones[(i, j)];
        }
    }
}

pub fn global_update_pheromones(pheromones: &mut FloatMatrix, ant: &Ant) {
    let d_tau = 1.0 / (ant.length as f64);
    for (&i, &j) in ant.tour.iter().tuple_windows() {
        pheromones[(i, j)] = pheromones[(i, j)] + d_tau;
        pheromones[(j, i)] = pheromones[(i, j)];
    }
}
