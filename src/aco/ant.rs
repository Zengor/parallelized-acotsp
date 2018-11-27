use indexmap::IndexSet;
use crate::util::Matrix;
use crate::instance_data::InstanceData;
use super::aco_parameters::AcoParameters;
use itertools::Itertools;

#[derive(Default, Clone)]
pub struct AntResult {
    pub tour: IndexSet<usize>,
    pub value: f64,
}

impl AntResult {
    fn new(data_size: usize) -> Self {
        AntResult {
            tour: IndexSet::with_capacity(data_size),
            value: 0.0f64,
        }
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

/// Form nearest neighbour tour given a starting city.
pub fn nearest_neighbour_tour(data: &InstanceData, starting_city: usize) -> AntResult {
    let mut result = AntResult::new(data.size);
    result.tour.insert(starting_city);
    let mut curr = starting_city;
    
    while result.tour.len() != data.size {
        // find smallest (index,value) not in in O(n)
        for (i,v) in data.distances[curr].iter().enumerate()
            .sorted_by(|a,b| PartialOrd::partial_cmp(a.1,b.1).unwrap()) {
                if result.tour.contains(&i) {
                    continue
                }               
                result.tour.insert(i);
                result.value += v;
                curr = i;
        }
        
    }
    result
}

pub fn mmas_ant(data: &InstanceData,
               pheromones: &Matrix,
               parameters: &AcoParameters) -> AntResult {
    unimplemented!()
}

