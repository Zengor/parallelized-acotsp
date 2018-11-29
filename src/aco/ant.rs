use indexmap::IndexSet;
use crate::util::{PheromoneMatrix};
use crate::instance_data::InstanceData;
use super::aco_parameters::AcoParameters;
use itertools::Itertools;

#[derive(Default, Clone)]
pub struct AntResult {
    pub tour: IndexSet<usize>,
    pub value: usize,
}

impl AntResult {
    fn new(data_size: usize) -> Self {
        AntResult {
            tour: IndexSet::with_capacity(data_size),
            value: 0,
        }
    }

    fn insert(&mut self, new_node: usize, connection_value: usize) {
        self.tour.insert(new_node);
        self.value += connection_value;
    }

    fn get_first(&self) -> usize {
        *self.tour.get_index(0).unwrap()
    }

    fn get_last(&self) -> usize {
        *self.tour.get_index(self.tour.len()-1).unwrap()
    }
}

#[derive(Default)]
struct Ant {
    result: AntResult,
    curr_city: usize,
}

impl Ant {
    fn new(data_size: usize) -> Self {
        Ant {
            result: AntResult::new(data_size),
            curr_city: 0
        }
    }
}

/// Form nearest neighbour tour given a starting city and return its value
pub fn nearest_neighbour_tour(data: &InstanceData, starting_city: usize) -> usize {
    let mut result = AntResult::new(data.size);
    result.tour.insert(starting_city);
    let mut curr = starting_city;
    let mut next = starting_city;
    let mut next_value = std::usize::MAX;
    while result.tour.len() != data.size {
        for (i,v) in data.distances[curr].iter().enumerate() {
            if !result.tour.contains(&i) && v < &next_value {
                next = i;
                next_value = *v;
            }
        }
        result.insert(next, next_value);
        curr = next;
        next_value = std::usize::MAX;
    }
    // Include edge between last and initial node in the value
    result.value += data.distances[result.get_last()][result.get_first()];
    result.value
}

pub fn mmas_ant(data: &InstanceData,
               pheromones: &PheromoneMatrix,
               parameters: &AcoParameters) -> AntResult {
    unimplemented!()
}

