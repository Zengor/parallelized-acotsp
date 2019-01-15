mod aco_parameters;
mod acs;
mod acspar;
mod ant;
mod colony;
mod mmas;
mod result_log;

use crate::instance_data::InstanceData;
use crate::util::IntegerMatrix;

pub use self::aco_parameters::{AcoParameters, Algorithm};
pub use self::ant::Ant;
use self::colony::Colony;
pub use self::result_log::{ResultLog, TimestampedResult};

pub fn run_aco(data: &InstanceData, parameters: &AcoParameters) -> ResultLog {
    let algorithm = &parameters.algorithm;

    match *algorithm {
        Algorithm::MMAS => {
            let colony = mmas::MMASColony::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::MMASPar => {
            let colony = mmas::MMASColony::initialize_parallel(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::ACS => {
            let colony = acs::ACSColony::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::ACSParMasterUpdate => {
            let colony = acs::ACSColony::initialize_parallel(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::ACSParSync => {
            let colony = acspar::ACSPar::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
    }
}

fn run_colony<'a>(
    mut colony: impl Colony<'a>,
    max_iterations: usize,
    max_time: usize,
) -> ResultLog {
    let mut result_log = ResultLog::new(max_iterations);
    while !check_termination(&colony, max_iterations, max_time) {
        colony.new_iteration();
        let results = colony.construct_solutions();
        update_stats(&results, &mut result_log, colony.iteration());
        colony.update_pheromones(result_log.latest_tour(), result_log.best_tour());
    }
    result_log
}

fn check_termination<'a>(colony: &impl Colony<'a>, max_iterations: usize, max_time: usize) -> bool {
    colony.iteration() > max_iterations || crate::timer::elapsed().as_secs() >= max_time as u64
}

fn update_stats(iter_results: &[Ant], result_log: &mut ResultLog, iteration: usize) {
    let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

fn find_best<'a>(results: &'a [Ant]) -> &'a Ant {
    results.iter().min_by_key(|x| x.length).unwrap()
}

pub fn total_value(pheromone: f64, heuristic: f64, alpha: f64, beta: f64) -> f64 {
    pheromone.powf(alpha) * heuristic.powf(beta)
}

fn heuristic(distances: &IntegerMatrix, i: usize, j: usize) -> f64 {
    1.0 / ((distances[i][j] as f64) + 0.1)
}
