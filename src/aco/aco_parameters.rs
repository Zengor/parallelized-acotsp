use super::Algorithm;

#[derive(Default)]
pub struct AcoParameters {
    pub num_ants: usize,
    /// Determines the influence of the pheromone trail
    pub alpha: f64,
    /// Determines the influence of the heuristic information
    pub beta: f64,
    /// Used to calculate rate at which pheromone evaporates in MMAS.
    /// Known as ρ in the literature
    pub evaporation_rate: f64,
    /// Probability an ant will make best possible move during tour construction in ACS
    pub q_0: f64,
    pub algorithm: Algorithm,
    /// Maximum number of iterations a colony may run
    pub max_iterations: usize,
    /// Maximum time in seconds that a colony may run
    pub time_limit: usize,
}
