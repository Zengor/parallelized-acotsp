use super::AcoParameters;
use super::ant::AntResult;
use crate::instance_data::InstanceData;

pub trait Colony<'a> {    
    fn initialize_colony(data: &'a InstanceData, parameters: &'a AcoParameters) -> Self;
    fn check_termination(&self) -> bool;
    fn new_iteration(&mut self);
    fn iteration(&self) -> usize;
    fn construct_solutions(&mut self) -> Vec<AntResult>;
    fn update_pheromones(&mut self, results: &Vec<AntResult>);
}

