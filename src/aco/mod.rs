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

/// Sets up and runs the algorithm.
pub fn run_aco(data: &InstanceData, parameters: &AcoParameters) -> ResultLog {
    let algorithm = &parameters.algorithm;
    // Colony could be turned into a trait object to avoid core repetition here,
    // but I'm not sure if it's worth it.
    // This structure was initially chosen to avoid the slight overhead of using trait
    // objects in the first place (though I think it'd ultimately not be that big of a deal.
    // Haven't actually tested it, though).
    // Since this is the only place that would be affected by a change in terms of
    // code length and readibility, I decided to leave it as is. If more colonies
    // were to be implemented, I'd probably change to using a trait object
    match *algorithm {
        Algorithm::Mmas => {
            let colony = mmas::MmasColony::initialize_colony(data, parameters, false);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::MmasPar => {
            let colony = mmas::MmasColony::initialize_colony(data, parameters, true);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::Acs => {
            let colony = acs::AcsColony::initialize_colony(data, parameters);
            run_colony(colony, parameters.max_iterations, parameters.time_limit)
        }
        Algorithm::AcsPar => {
            let colony = acspar::AcsPar::initialize_colony(data, parameters);
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

/// Function to calculate the total weight of a given connection, combining its current
/// pheromones and the inherit heuristic information, modified by algorithm parameters
/// alpha and beta.
pub fn total_value(pheromone: f64, heuristic: f64, alpha: f64, beta: f64) -> f64 {
    pheromone.powf(alpha) * heuristic.powf(beta)
}

fn heuristic(distances: &IntegerMatrix, i: usize, j: usize) -> f64 {
    1.0 / ((distances[(i, j)] as f64) + 0.1)
}
