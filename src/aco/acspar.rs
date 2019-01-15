use itertools::Itertools;
use rayon::prelude::*;

use crate::instance_data::InstanceData;
use crate::util::{self, FloatMatrix, FloatMatrixSync};

use super::ant;
use super::colony::{compute_combined_info, Colony};
use super::AcoParameters;
use super::Ant;

use std::sync::Arc;

pub struct ACSPar<'a> {
    iteration: usize,
    data: &'a InstanceData,
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

    fn construct_solutions(&mut self) -> Vec<Ant> {
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
                        )
                    },
                    |(pheromones, combined_info), ant| {
                        let ant = {
                            ant::acs_ant_step_sync(
                                ant,
                                self.data,
                                &self.combined_info,
                                self.parameters,
                            )
                        };
                        local_pheromone_update(
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
            ant.length += self.data.distances[ant.get_last()][ant.get_first()];
        }
        ants_vec
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
            // this method is always run on the main thread, there's no need to worry about how long we hold the locks
            let mut comb_ij = self.combined_info[i][j].write();
            let mut comb_ji = self.combined_info[j][i].write();
            let mut pherom_ij = self.pheromones[i][j].write();
            let mut pherom_ji = self.pheromones[j][i].write();
            *pherom_ij = coefficient * *pherom_ij + evap_rate * d_tau;
            *pherom_ji = *pherom_ij;
            *comb_ij = super::total_value(
                *self.pheromones[i][j].read(),
                self.heuristic_info[i][j],
                alpha,
                beta,
            );
            *comb_ji = *comb_ij;
        }
    }
}

fn calculate_initial_values(nn_tour_length: u32, num_nodes: usize) -> f64 {
    1.0 / (num_nodes * nn_tour_length as usize) as f64
}

fn local_pheromone_update(
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
    // grabbing locks for the combined info early to avoid another thread updating them before we're done
    // with calculating the value that we'll modify
    let mut comb_ij = combined_info[i][j].write();
    let mut comb_ji = combined_info[j][i].write();
    {
        // grabbing the pheromone locks in an inner scope so we can free it as soon as we're done,
        // so another thread needing to just read it can keep going
        let mut pherom_ij = pheromones[i][j].write();
        let mut pherom_ji = pheromones[j][i].write();
        // calculating new pheromone value
        let modified_old_pherom = (1.0 - xi) * *pherom_ij;
        let added_pherom = xi * initial_trail;
        *pherom_ij = modified_old_pherom + added_pherom;
        *pherom_ji = *pherom_ij;
    }
    *comb_ij = super::total_value(*pheromones[i][j].read(), heuristic_info[i][j], alpha, beta);
    *comb_ji = *comb_ij;
}
