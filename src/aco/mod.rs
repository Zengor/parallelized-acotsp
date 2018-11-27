mod aco_parameters;
mod ant;
mod mmas;
mod acs;
mod result_log;

pub use self::aco_parameters::AcoParameters;

use crate::util::Matrix;
use crate::instance_data::InstanceData;
pub use self::ant::AntResult;
pub use self::result_log::{TimestampedResult, ResultLog};
pub enum Algorithm {
    ACS,
    MMAS,
}

impl Default for Algorithm {
    fn default() -> Algorithm { Algorithm::ACS }
}

pub struct Colony<'a> {
    iteration: usize,
    data: &'a InstanceData,
    pheromones: Matrix,
    nn_list: Vec<AntResult>,
    parameters: &'a AcoParameters,
}

fn check_termination() -> bool {
    unimplemented!()
}

fn run(data: &InstanceData, parameters: &mut AcoParameters) {
    //initialize timer and logger
    //initialize pheromones
    //initialize nn_lists
    let mut results_log = ResultLog::new(parameters.max_iterations);
    let mut colony = initialize_colony(data, parameters);
    while !check_termination() {
        colony.iteration += 1;
        let results = construct_solutions(&mut colony);
        update_stats(&results, &mut results_log, colony.iteration);
        update_pheromones(&mut colony, results);
    }
}

fn initialize_colony<'a>(data: &'a InstanceData, parameters: &'a mut AcoParameters) -> Colony<'a> {
    let mut nn_list = Vec::with_capacity(data.size);
    for i in 0.. data.size {
        nn_list.push(ant::nearest_neighbour_tour(data, i));
    }
    
    parameters.pheromone_initial = nn_list[0].value;
    
    let mut colony = Colony {
        iteration: 0,
        data,
        pheromones: crate::util::generate_empty_matrix(data.size),
        nn_list,
        parameters,
    };
    colony
}

fn construct_solutions(colony: &mut Colony) -> Vec<AntResult> {
    match colony.parameters.algorithm {
        Algorithm::MMAS => mmas::construct(colony),
        Algorithm::ACS => construct_acs(colony),
    }
}


fn construct_acs(colony: &mut Colony) -> Vec<AntResult> {
    unimplemented!()
}

fn update_stats(iter_results: &[AntResult], result_log: &mut ResultLog, iteration: usize) {
    let best_this_iter = find_best(iter_results);
    result_log.push(best_this_iter.to_owned(), iteration);
}

fn find_best<'a>(results: &'a [AntResult]) -> &'a AntResult {
    results.iter().min_by(|x,y| PartialOrd::partial_cmp(&x.value,&y.value).unwrap()).unwrap()
}

fn update_pheromones(colony: &mut Colony, results: Vec<AntResult>) {
    //update statistical info (best so far)
    //then update pheromones proper
    match colony.parameters.algorithm {
        Algorithm::MMAS => mmas::update_pheromones(colony, &results),
        Algorithm::ACS => unimplemented!(),
    }  
}
