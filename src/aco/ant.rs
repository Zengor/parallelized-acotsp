use super::aco_parameters::AcoParameters;
use crate::instance_data::InstanceData;
use crate::util::ColonyInfoMatrix;
use indexmap::IndexSet;
use rand::{thread_rng, Rng};

#[derive(Default, Clone, Debug)]
pub struct Ant {
    pub tour: IndexSet<usize>,
    pub length: u32,
    pub curr_city: usize,
}

impl Ant {
    fn new(data_size: usize) -> Self {
        Ant {
            tour: IndexSet::with_capacity(data_size),
            length: 0,
            curr_city: 0,
        }
    }

    fn new_on_city(data_size: usize, starting_city: usize) -> Self {
        let mut a = Self::new(data_size);
        a.insert(starting_city, 0);
        a
    }

    // pub fn drain_to_result(&mut self) -> AntResult {
    //     let tour = self.tour.drain(..).collect();
    //     self.length = 0;
    //     self.curr_city = 0;

    //     AntResult {
    //         tour,
    //         length: self.length,
    //     }
    // }

    fn insert(&mut self, new_node: usize, connection_length: u32) {
        //println!("{},{}: {} + {}", self.curr_city, new_node, self.length, connection_length );
        self.curr_city = new_node;
        self.tour.insert(new_node);
        self.length += connection_length;
    }

    pub fn get_first(&self) -> usize {
        *self.tour.get_index(0).expect("get first")
    }

    pub fn get_last(&self) -> usize {
        *self.tour.get_index(self.tour.len() - 1).expect("get_last")
    }

    pub fn get_last_arc(&self) -> (usize, usize) {
        (
            *self.tour.get_index(self.tour.len() - 2).expect("last arc"),
            self.get_last(),
        )
    }
}

/// Form nearest neighbour tour given a starting city and return its length
pub fn nearest_neighbour_tour(data: &InstanceData, starting_city: usize) -> u32 {
    let mut tour = IndexSet::with_capacity(data.size);
    tour.insert(starting_city);
    let mut curr = starting_city;
    let mut next = starting_city;
    let mut next_length = std::u32::MAX;
    let mut length = 0;
    while tour.len() != data.size {
        for (i, v) in data.distances.row(curr).iter().enumerate() {
            if !tour.contains(&i) && v < &next_length {
                next = i;
                next_length = *v;
            }
        }

        tour.insert(next);
        length += next_length;
        curr = next;
        next_length = std::u32::MAX;
    }
    // Include edge between last and initial node in the length
    length += data.distances[(tour.pop().unwrap(), *tour.get_index(0).unwrap())];
    length
}

/// Chooses an unvisited city to go to with the highest total combined heuristic+pheromone
/// information. Returns the index of that city.
fn choose_best_next<T: ColonyInfoMatrix>(
    curr_city: usize,
    visited: &IndexSet<usize>,
    combined_info: &T,
) -> usize {
    combined_info.filtered_row_max(curr_city, visited)
}

/// Chooses an unvisited city to go to using the proportional rule defined in the literature.
/// Returns the index of that city.
fn choose_probabilistically<T: ColonyInfoMatrix>(
    curr_city: usize,
    visited: &IndexSet<usize>,
    combined_info: &T,
    rng: &mut impl Rng,
) -> usize {
    let (unvisited, weights) = combined_info.unvisited_weights(curr_city, visited);
    let mut random_v: f64 = rng.gen::<f64>() * weights.iter().sum::<f64>();
    for (city, weight) in unvisited.iter().zip(weights) {
        random_v -= weight;
        if random_v < 0.0 {
            return *city;
        }
    }
    unreachable!()
}

pub fn mmas_ant<T: ColonyInfoMatrix>(data: &InstanceData, combined_info: &T) -> Ant {
    let mut rng = thread_rng();
    let starting_city = rng.gen_range(0, data.size);
    let mut ant = Ant::new_on_city(data.size, starting_city);
    //ant.length = 0;
    for _ in 0..data.size - 1 {
        let next_city = choose_probabilistically(ant.curr_city, &ant.tour, combined_info, &mut rng);
        //TODO use nn_list to aid performance
        //let next_city = choose_best_next(ant.curr_city, &ant.tour, combined_info);
        ant.insert(next_city, data.distances[(ant.curr_city, next_city)]);
    }
    // Include edge between last and initial node in the length
    ant.length += data.distances[(ant.get_last(), ant.get_first())];
    ant
}

/// Generates a vec of `Ant`s. To be used with ACS so it can step each ant individually and
/// update pheromones locally. Ants are placed on a random initial city.
pub fn create_ants(num_ants: usize, data_size: usize) -> Vec<Ant> {
    let mut v = Vec::with_capacity(num_ants);
    for _ in 0..num_ants {
        let starting_city = thread_rng().gen_range(0, data_size);
        v.push(Ant::new_on_city(data_size, starting_city));
    }
    v
}

/// A single step for an `Ant` in the ACS algorithm.
/// Returns an ant that has moved a step further
pub fn acs_ant_step<T: ColonyInfoMatrix>(
    mut ant: Ant,
    data: &InstanceData,
    combined_info: &T,
    parameters: &AcoParameters,
) -> Ant {
    // note: acs assumes an aplha value of 1 in all cases
    let mut rng = thread_rng();
    let next_city = if rng.gen_bool(parameters.q_0) {
        // get max heuristic info
        choose_best_next(ant.curr_city, &ant.tour, combined_info)
    } else {
        //get probabilistic
        choose_probabilistically(ant.curr_city, &ant.tour, combined_info, &mut rng)
    };
    ant.insert(next_city, data.distances[(ant.curr_city, next_city)]);
    ant
}
