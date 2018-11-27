use itertools::Itertools;

use super::Colony;
use super::AntResult;
use super::ant::mmas_ant;

pub fn construct(colony: &mut Colony) -> Vec<AntResult> {
    let n_ants = colony.parameters.num_ants;
    (0..n_ants).into_iter()
        .map(|_| mmas_ant(colony.data, &colony.pheromones, colony.parameters))
        .collect()
}


pub fn update_pheromones(colony: &mut Colony, results: &Vec<AntResult>) {
    evaporate(colony);
    
    for a in results { 
        for (&i,&j) in a.tour.iter().tuple_windows() {
            colony.pheromones[i][j] += 2.0; //TODO
            colony.pheromones[j][i] += 2.0;
        }
    }
}



fn evaporate(colony: &mut Colony) {
    let evap_rate = colony.parameters.evaporation_rate;
    let trail_min = colony.parameters.trail_min;
    for i in 0..colony.data.size {
        for j in 0..i {
            let mut new_pheromone = (1.0 - evap_rate) * colony.pheromones[i][j];
            if new_pheromone < trail_min {
                new_pheromone = trail_min;
            }
            colony.pheromones[i][j] = new_pheromone;
            colony.pheromones[j][i] = new_pheromone;
        }
    }
}
