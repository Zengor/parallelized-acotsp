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
    // This section has a bit of code repetition. This could be reduced by using
    // a Box<Colony>, but the Colony type can't be turned into a trait object. I'm not 100%
    // sure on why, but I believe it's because of the lifetime parameter on the references.
    // I could change the structure so that Colony owns clones of the InstanceData and the
    // Parameters, but since it'd only fix code repetition in this small section,
    // I'm not sure it's worth it.
    // This structure was also initially chosen to avoid the slight overhead of using trait
    // objects in the first place (though I think it'd ultimately not be that big of a deal.
    // Haven't actually tested it, though).
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
        let result = colony.construct_solutions();
        update_stats(result, &mut result_log, colony.iteration());
        colony.update_pheromones(result_log.latest_tour(), result_log.best_tour());
    }
    result_log
}

fn check_termination<'a>(colony: &impl Colony<'a>, max_iterations: usize, max_time: usize) -> bool {
    colony.iteration() > max_iterations || crate::timer::elapsed().as_secs() >= max_time as u64
}

fn update_stats(best_this_iter: Ant, result_log: &mut ResultLog, iteration: usize) {
    //let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

#[allow(dead_code)]
fn find_best<'a>(results: &'a [Ant]) -> &'a Ant {
    results.iter().min_by_key(|x| x.length).unwrap()
}

pub fn total_value(pheromone: f64, heuristic: f64, alpha: f64, beta: f64) -> f64 {
    pheromone.powf(alpha) * heuristic.powf(beta)
}

fn heuristic(distances: &IntegerMatrix, i: usize, j: usize) -> f64 {
    1.0 / ((distances[(i, j)] as f64) + 0.1)
}
