mod aco_parameters;
mod ant;
mod mmas;
mod acs;
mod result_log;
mod colony;

use itertools::Itertools;
use crate::util::{PheromoneMatrix};
use crate::instance_data::InstanceData;

pub use self::aco_parameters::AcoParameters;
pub use self::ant::AntResult;
pub use self::result_log::{TimestampedResult, ResultLog};
use self::colony::Colony;

pub enum Algorithm {
    ACS,
    MMAS,
}

impl Default for Algorithm {
    fn default() -> Algorithm { Algorithm::ACS }
}

/// Returns true if run should end
fn check_termination() -> bool {
    unimplemented!()
}

fn run_aco(data: &InstanceData, parameters: &mut AcoParameters) {
    let algorithm = &parameters.algorithm;
    let max_iterations = parameters.max_iterations;
    match *algorithm {
        Algorithm::MMAS => {
            let colony = mmas::MMASColony::initialize_colony(data, parameters);
            run_colony(colony, max_iterations)
        },
        Algorithm::ACS => {

        }
    }
}

fn run_colony<'a>(mut colony: impl Colony<'a>, max_iterations: usize) {
    //initialize timer and logger
    //initialize pheromones
    //initialize nn_lists
    let mut results_log = ResultLog::new(max_iterations);
    while !check_termination() {
        colony.new_iteration();
        let results = colony.construct_solutions();
        update_stats(&results, &mut results_log, colony.iteration());
        colony.update_pheromones(&results);
    }
}

fn generate_nn_list(data: &InstanceData) -> Vec<Vec<usize>>{
    let mut nn_list = Vec::with_capacity(data.size);
    for i in 0..data.size {
        let mut sorted = data.distances[i].iter()
                                        .map(|x| x.to_owned())
                                        .sorted();
        sorted.pop();
        nn_list.push(sorted);
    }
    nn_list
}

/// Calculates initial pheromone trails, as well as trail_max and trail_min for MMAS
fn calculate_initial_values(nn_tour_length: usize,
                            num_nodes: usize,
                            parameters: &mut AcoParameters) {
    match &parameters.algorithm {
        Algorithm::MMAS => {
            parameters.trail_max = 1.0 / (parameters.evaporation_rate * nn_tour_length as f64);
            parameters.trail_min = parameters.trail_max / (2.0 * num_nodes as f64);
            parameters.pheromone_initial = parameters.trail_max;
        },
        Algorithm::ACS => {
            parameters.pheromone_initial = 1.0 / (num_nodes * nn_tour_length) as f64;
        }
    }
}


fn update_stats(iter_results: &[AntResult], result_log: &mut ResultLog, iteration: usize) {
    let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

fn find_best<'a>(results: &'a [AntResult]) -> &'a AntResult {
    results.iter().min_by(|x,y| PartialOrd::partial_cmp(&x.value,&y.value).unwrap()).unwrap()
}
