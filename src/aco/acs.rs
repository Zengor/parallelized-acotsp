use itertools::Itertools;

use crate::util::{self, FloatMatrix};
use crate::instance_data::InstanceData;

use super::ant;
use super::colony::{Colony, compute_combined_info};
use super::AcoParameters;
use super::Ant;


pub struct ACSColony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: FloatMatrix,
    /// Heuristic information based on the distance, calculated on initialization
    heuristic_info: FloatMatrix,
    /// Combined pheromone + heuristic information, recalculated every iteration
    combined_info: FloatMatrix,
    //nn_list: Vec<Vec<usize>>,
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
        for _ in 0..data_size {
            for ant in ants_vec.iter_mut() {
                ant::acs_ant_step(ant, self.data, &self.combined_info, self.parameters);
                //local pheromone update here
                self.local_pheromone_update(ant);
            }
        }
        ants_vec
    }
    
    fn update_pheromones(&mut self, _: &Ant, best_so_far: &Ant) {
        global_update_pheromones(&mut self.pheromones, &mut self.heuristic_info, &mut self.combined_info, self.parameters, best_so_far);
    }
}

impl ACSColony<'_> {
    fn local_pheromone_update(&mut self, ant: &ant::Ant) {
        let (i,j) = ant.get_last_arc();
        let modified_old_pherom = (1.0 - self.parameters.xi) * self.pheromones[i][j];
        let added_pherom = self.parameters.xi * self.initial_trail;
        self.pheromones[i][j] = modified_old_pherom + added_pherom;
    }
}

fn calculate_initial_values(nn_tour_length: u32, num_nodes: usize) -> f64 {
    1.0 / (num_nodes * nn_tour_length as usize) as f64
}

fn global_update_pheromones(pheromones: &mut FloatMatrix, 
                            heuristic_info: &mut FloatMatrix,
                            combined_info: &mut FloatMatrix,
                            parameters: &AcoParameters,
                            best_so_far: &Ant) {
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
