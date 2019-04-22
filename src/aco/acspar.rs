use crate::instance_data::InstanceData;
use crate::util::{self, FloatMatrix, FloatMatrixSync};
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

use super::ant;
use super::colony::{compute_combined_info, Colony};
use super::AcoParameters;
use super::Ant;

use std::sync::Arc;

pub struct ACSPar<'a> {
    iteration: usize,
    data: &'a InstanceData,
    lock_mutex: Arc<Mutex<()>>,
    pheromones: FloatMatrixSync,
    /// Heuristic information based on the distance, calculated on initialization
    heuristic_info: FloatMatrix,
    /// Combined pheromone + heuristic information, recalculated every iteration
    combined_info: FloatMatrixSync,
    //nn_list: Vec<Vec<usize>>,
    initial_trail: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for ACSPar<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> ACSPar<'a> {
        let nn_tour_length = ant::nearest_neighbour_tour(data, 0);
        let initial_trail = calculate_initial_values(nn_tour_length, data.size);
        let pheromones = util::generate_pheromone_matrix(data.size, initial_trail);
        let (heuristic_info, combined_info) =
            compute_combined_info(&data.distances, &pheromones, parameters);
        let pheromones = util::convert_to_sync(pheromones);
        let combined_info = util::convert_to_sync(combined_info);
        Self {
            iteration: 0,
            data,
            lock_mutex: Arc::new(Mutex::new(())),
            pheromones,
            heuristic_info,
            combined_info,
            //nn_list: super::generate_nn_list(data),
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

    fn construct_solutions(&mut self) -> Ant {
        let n_ants = self.parameters.num_ants;
        let data_size = self.data.size;
        let mut ants_vec = ant::create_ants(n_ants, data_size);

        for _ in 0..data_size - 1 {
            ants_vec = ants_vec
                .into_par_iter()
                .map_init(
                    || {
                        (
                            Arc::clone(&self.pheromones),
                            Arc::clone(&self.combined_info),
                            Arc::clone(&self.lock_mutex),
                        )
                    },
                    |(pheromones, combined_info, mutex), ant| {
                        let ant = ant::acs_ant_step_sync(
                            ant,
                            self.data,
                            &self.combined_info,
                            self.parameters,
                        );
                        local_pheromone_update(
                            mutex,
                            pheromones,
                            &self.heuristic_info,
                            combined_info,
                            self.parameters,
                            self.initial_trail,
                            &ant,
                        );
                        ant
                    },
                )
                .collect();
        }
        for ant in ants_vec.iter_mut() {
            ant.length += self.data.distances[(ant.get_last(), ant.get_first())];
        }
        ants_vec.into_iter().min_by_key(|a| a.length).unwrap()
    }

    fn update_pheromones(&mut self, _: &Ant, best_so_far: &Ant) {
        let d_tau = 1.0 / best_so_far.length as f64;
        let (alpha, beta, evap_rate) = (
            self.parameters.alpha,
            self.parameters.beta,
            self.parameters.evaporation_rate,
        );
        let coefficient = 1.0 - evap_rate;
        for (&i, &j) in best_so_far.tour.iter().tuple_windows() {
            // this method is always run on the main thread, there's no need to worry
            // about avoiding deadlocks by using the mutex before getting the locks
            let mut comb_ij = self.combined_info[(i, j)].write();
            let mut comb_ji = self.combined_info[(j, i)].write();
            let mut pherom_ij = self.pheromones[(i, j)].write();
            let mut pherom_ji = self.pheromones[(j, i)].write();
            *pherom_ij = coefficient * *pherom_ij + evap_rate * d_tau;
            *pherom_ji = *pherom_ij;
            *comb_ij = super::total_value(*pherom_ij, self.heuristic_info[(i, j)], alpha, beta);
            *comb_ji = *comb_ij;
        }
    }
}

fn calculate_initial_values(nn_tour_length: u32, num_nodes: usize) -> f64 {
    1.0 / (num_nodes * nn_tour_length as usize) as f64
}

fn local_pheromone_update(
    mutex: &mut Arc<Mutex<()>>,
    pheromones: &mut FloatMatrixSync,
    heuristic_info: &FloatMatrix,
    combined_info: &mut FloatMatrixSync,
    parameters: &AcoParameters,
    initial_trail: f64,
    ant: &Ant,
) {
    let (i, j) = ant.get_last_arc();
    // making them local variables for convenience and readability
    let (alpha, beta, xi) = (parameters.alpha, parameters.beta, parameters.xi);
    let (mut comb_ij, mut comb_ji, mut pherom_ij, mut pherom_ji) = {
        // this mutex lock is necessary because we might have two threads going for (i,j) and (j,i) separetely
        let _lock = mutex.lock();
        let comb_ij = combined_info[(i, j)].write();
        let comb_ji = combined_info[(j, i)].write();
        let pherom_ij = pheromones[(i, j)].write();
        let pherom_ji = pheromones[(j, i)].write();
        (comb_ij, comb_ji, pherom_ij, pherom_ji)
    };
    let modified_old_pherom = (1.0 - xi) * *pherom_ij;
    let added_pherom = xi * initial_trail;
    *pherom_ij = modified_old_pherom + added_pherom;
    *pherom_ji = *pherom_ij;
    *comb_ij = super::total_value(*pherom_ij, heuristic_info[(i, j)], alpha, beta);
    *comb_ji = *comb_ij;
}
