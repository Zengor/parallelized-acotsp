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
    MMAS,
    ACS,
}

impl Default for Algorithm {
    fn default() -> Algorithm { Algorithm::MMAS }
}

<<<<<<< HEAD
fn run_aco(data: &InstanceData, parameters: &AcoParameters) -> ResultLog {
=======
pub fn run_aco(data: &InstanceData, parameters: &AcoParameters) -> ResultLog{
>>>>>>> e5a2c13f129a85fa8fd1182200fad9aff6842017
    let algorithm = &parameters.algorithm;

    match *algorithm {
        Algorithm::MMAS => {
            let colony = mmas::MMASColony::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations)
        },
        Algorithm::ACS => {
            unimplemented!()
        }
    }
}

<<<<<<< HEAD
fn run_colony<'a>(mut colony: impl Colony<'a>, max_iterations: usize) -> ResultLog{
    //initialize timer and logger
    //initialize pheromones
    //initialize nn_lists
    let mut results_log = ResultLog::new(max_iterations);
    while !colony.check_termination() {
=======
fn run_colony<'a>(mut colony: impl Colony<'a>, max_iterations: usize) -> ResultLog {
    let mut result_log = ResultLog::new(max_iterations);
    while !check_termination(&colony, max_iterations) {
>>>>>>> e5a2c13f129a85fa8fd1182200fad9aff6842017
        colony.new_iteration();
        let results = colony.construct_solutions();
        update_stats(&results, &mut result_log, colony.iteration());
        colony.update_pheromones(result_log.latest_tour(), result_log.best_tour());
    }
<<<<<<< HEAD
    results_log
=======
    result_log
}

fn check_termination<'a>(colony: &impl Colony<'a>, max_iterations: usize) -> bool {
    colony.iteration() > max_iterations
>>>>>>> e5a2c13f129a85fa8fd1182200fad9aff6842017
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
