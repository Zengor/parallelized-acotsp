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
    //evaporate
    //then
    
    for a in results { 
        for (&i,&j) in a.tour.iter().tuple_windows() {
            colony.pheromones[i][j] += 2.0; //TODO
            colony.pheromones[j][i] += 2.0;
        }
    }
}
