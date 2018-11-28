use itertools::Itertools;

use super::colony::Colony;
use super::AntResult;
use super::ant::mmas_ant;
use super::AcoParameters;
use crate::instance_data::InstanceData;
use crate::util::{PheromoneMatrix};
pub struct MMASColony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: PheromoneMatrix,
    nn_list: Vec<Vec<usize>>,
    /// Maximum pheromone value for MMAS. This is calculated by the colony.
    pub trail_max: f64,
    /// Minimum pheromone value for MMAS. This is calculated by the colony.
    pub trail_min: f64,
    parameters: &'a AcoParameters,
}

impl<'a> Colony<'a> for MMASColony<'a> {
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> MMASColony<'a> {
        let nn_tour_length = super::ant::nearest_neighbour_tour(data, 0);
        //let mut nn_list = Vec::with_capacity(data.size);
        // TODO calculate nn_list
    
        let (trail_min, trail_max) = MMASColony::calculate_initial_values(nn_tour_length, 
                                                                          data.size, 
                                                                          parameters);
    
        Self {
            iteration: 0,
            data,
            pheromones: crate::util::generate_pheromone_matrix(data.size, trail_max),
            nn_list: super::generate_nn_list(data),
            trail_max,
            trail_min,
            parameters,
        }
    }

    fn check_termination(&self) -> bool {
        unimplemented!()
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
    
    
    fn update_pheromones(&mut self, results: &Vec<AntResult>) {
        self.evaporate();
    
        for a in results { 
            for (&i,&j) in a.tour.iter().tuple_windows() {
                self.pheromones[i][j] += 2.0; //TODO
                self.pheromones[j][i] += 2.0;
            }
        }   
    }
}

impl MMASColony<'_> {
    /// Calculates initial pheromone trails, as well as trail_max and trail_min for MMAS
    fn calculate_initial_values(nn_tour_length: usize,
                                num_nodes: usize,
                                parameters: &AcoParameters) -> (f64, f64) {
        let trail_max = 1.0 / (parameters.evaporation_rate * nn_tour_length as f64);
        let trail_min = trail_max / (2.0 * num_nodes as f64);
        (trail_min, trail_max)
    }

    fn evaporate(&mut self) {
        let evap_rate = self.parameters.evaporation_rate;
        let trail_min = self.trail_min;
        for i in 0..self.data.size {
            for j in 0..i {
                let mut new_pheromone = (1.0 - evap_rate) * self.pheromones[i][j];
                if new_pheromone < trail_min {
                    new_pheromone = trail_min;
                }
                self.pheromones[i][j] = new_pheromone;
                self.pheromones[j][i] = new_pheromone;
            }
        }
    }
}







