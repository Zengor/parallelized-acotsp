mod aco_parameters;
mod ant;
mod mmas;
mod acs;
mod result_log;
mod colony;

use itertools::Itertools;
use crate::util::{PheromoneMatrix, IntegerMatrix};
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

pub fn run_aco(data: &InstanceData, parameters: &AcoParameters) {
    let algorithm = &parameters.algorithm;

    match *algorithm {
        Algorithm::MMAS => {
            let colony = mmas::MMASColony::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations)
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
    while !colony.check_termination() {
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


fn update_stats(iter_results: &[AntResult], result_log: &mut ResultLog, iteration: usize) {
    let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

fn find_best<'a>(results: &'a [AntResult]) -> &'a AntResult {
    results.iter().min_by(|x,y| PartialOrd::partial_cmp(&x.length,&y.length).unwrap()).unwrap()
}

pub fn total_value(distances: &IntegerMatrix, pheromones: &PheromoneMatrix,
               parameters: &AcoParameters, i: usize, j: usize) -> f64 {
    pheromones[i][j].powf(parameters.alpha) * heuristic(distances, i, j).powf(parameters.beta)
}

fn heuristic(distances: &IntegerMatrix, i: usize, j: usize) -> f64 {
    1.0 / ((distances[i][j] as f64) + 0.1)
}
