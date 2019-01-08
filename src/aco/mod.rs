mod aco_parameters;
mod ant;
mod mmas;
mod acs;
mod result_log;
mod colony;

use itertools::Itertools;
use crate::util::{FloatMatrix, IntegerMatrix};
use crate::instance_data::InstanceData;

pub use self::aco_parameters::{AcoParameters, Algorithm};
pub use self::ant::Ant;
pub use self::result_log::{TimestampedResult, ResultLog};
use self::colony::Colony;

pub fn run_aco(data: &InstanceData, parameters: &AcoParameters) -> ResultLog{
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

fn run_colony<'a>(mut colony: impl Colony<'a>, max_iterations: usize) -> ResultLog {
    let mut result_log = ResultLog::new(max_iterations);
    while !check_termination(&colony, max_iterations) {
        colony.new_iteration();
        let results = colony.construct_solutions();
        update_stats(&results, &mut result_log, colony.iteration());
        colony.update_pheromones(result_log.latest_tour(), result_log.best_tour());
    }
    result_log
}

fn check_termination<'a>(colony: &impl Colony<'a>, max_iterations: usize) -> bool {
    colony.iteration() > max_iterations
}

fn generate_nn_list(data: &InstanceData) -> Vec<Vec<u32>>{
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


fn update_stats(iter_results: &[Ant], result_log: &mut ResultLog, iteration: usize) {
    let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

fn find_best<'a>(results: &'a [Ant]) -> &'a Ant {
    results.iter().min_by_key(|x| x.length).unwrap()
}

pub fn total_value(pheromone: f64,
                   heuristic: f64,
                   alpha: f64,
                   beta: f64) -> f64 {
    pheromone.powf(alpha) * heuristic.powf(beta)
}

fn heuristic(distances: &IntegerMatrix, i: usize, j: usize) -> f64 {
    1.0 / ((distances[i][j] as f64) + 0.1)
}
