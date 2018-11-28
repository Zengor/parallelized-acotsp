use super::AcoParameters;
use super::ant::AntResult;
use crate::instance_data::InstanceData;

pub trait Colony<'a> {    
    fn initialize_colony(data: &'a InstanceData, parameters: &'a mut AcoParameters) -> Self;
    fn check_termination() -> bool;
    fn iteration(&self) -> usize;
    fn construct_solutions(&mut self) -> Vec<AntResult>;
    fn update_pheromones(&mut self, results: &Vec<AntResult>);
}

