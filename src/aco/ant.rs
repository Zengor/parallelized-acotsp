use indexmap::IndexSet;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use crate::util::{PheromoneMatrix, IntegerMatrix};
use crate::instance_data::InstanceData;
use super::aco_parameters::AcoParameters;

#[derive(Default, Clone, Debug)]
pub struct AntResult {
    pub tour: Vec<usize>,
    pub length: usize,
}

impl AntResult {
    fn new(data_size: usize) -> Self {
        AntResult {
            tour: Vec::with_capacity(data_size),
            length: 0,
        }
    }
}

#[derive(Default)]
struct Ant {
    tour: IndexSet<usize>,
    length: usize,
    curr_city: usize,
}

impl Ant {
    fn new(data_size: usize) -> Self {
        Ant {
            tour: IndexSet::with_capacity(data_size),
            length: 0,
            curr_city: 0,
        }
    }
    
    fn drain_to_result(&mut self) -> AntResult {
        let tour = self.tour.drain(..).collect();
        AntResult {
            tour,
            length: self.length,
        }
    }
    
    fn insert(&mut self, new_node: usize, connection_length: usize) {
        self.tour.insert(new_node);
        self.length += connection_length;
    }

    fn get_first(&self) -> usize {
        *self.tour.get_index(0).unwrap()
    }

    fn get_last(&self) -> usize {
        *self.tour.get_index(self.tour.len()-1).unwrap()
    }
}

/// Form nearest neighbour tour given a starting city and return its length
pub fn nearest_neighbour_tour(data: &InstanceData, starting_city: usize) -> usize {
    let mut tour = IndexSet::with_capacity(data.size);
    tour.insert(starting_city);
    let mut curr = starting_city;
    let mut next = starting_city;
    let mut next_length = std::usize::MAX;
    let mut length = 0;
    while tour.len() != data.size {
        for (i,v) in data.distances[curr].iter().enumerate() {
            if !tour.contains(&i) && v < &next_length {
                next = i;
                next_length = *v;
            }
        }
        tour.insert(next);
        length += next_length;
        curr = next;
        next_length = std::usize::MAX;
    }
    // Include edge between last and initial node in the length
    length += data.distances[tour.pop().unwrap()][*tour.get_index(0).unwrap()];
    length
}

fn choose_best_next(curr_city: usize,
                    visited: &IndexSet<usize>,
                    combined_info: &PheromoneMatrix) -> usize {
    let (next_city,_) = combined_info[curr_city]
        .iter()
        .enumerate()
        .filter(|(city,_)| !visited.contains(city))
        .max_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap()).unwrap();
    next_city
}

pub fn global_update_pheromones(pheromones: &mut PheromoneMatrix, ant: &AntResult) {
    let d_tau = 1.0 / (ant.length as f64);
    for (&i,&j) in ant.tour.iter().tuple_windows() {
        pheromones[i][j] += d_tau;
        pheromones[j][i] =  pheromones[i][j];
    }   
}

pub fn mmas_ant(data: &InstanceData,
                combined_info: &PheromoneMatrix,
                parameters: &AcoParameters) -> AntResult {
    let mut rng = thread_rng();
    let mut curr_city: usize = rng.gen_range(0, data.size);
    let mut ant = Ant::new(data.size);
    ant.tour.insert(curr_city);
    //ant.length = 0;
    for _ in 0..data.size-1 {
        //TODO use nn_list to aid performance
        let next_city = choose_best_next(curr_city, &ant.tour, combined_info);
        ant.insert(next_city, data.distances[curr_city][next_city]);
        curr_city = next_city;
    }
    // Include edge between last and initial node in the length
    ant.length += data.distances[ant.get_last()][ant.get_first()];
    ant.drain_to_result()
}
