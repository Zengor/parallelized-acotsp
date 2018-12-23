use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Algorithm {
    MMAS,
    ACS,
}

impl Default for Algorithm {
    fn default() -> Algorithm { Algorithm::MMAS }
}

#[derive(Default, Deserialize)]
pub struct AcoParameters {
    #[serde(default = "default_num_ants")]
    pub num_ants: usize,
    /// Determines the influence of the pheromone trail
    #[serde(default = "default_alpha")]
    pub alpha: f64,
    /// Determines the influence of the heuristic information
    #[serde(default = "default_beta")]
    pub beta: f64,
    /// Used to calculate rate at which pheromone evaporates in MMAS.
    /// Known as ρ in the literature
    #[serde(default = "default_evap")]
    pub evaporation_rate: f64,
    /// Probability an ant will make best possible move during tour construction in ACS
    #[serde(default = "default_q0")]
    pub q_0: f64,
    /// Used to calculate coefficients in local pheromone update in ACS
    /// (old pheromone is (1-xi), new is xi). Between 0 and 1.
    #[serde(default = "default_xi")]
    pub xi: f64, 
    pub algorithm: Algorithm,
    /// Maximum number of iterations a colony may run
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,
    /// Maximum time in seconds that a colony may run
    #[serde(default = "default_time_limit")]
    pub time_limit: usize,
}

fn default_num_ants () -> usize { 10 }
fn default_alpha() -> f64 { 1.0 }
fn default_beta() -> f64 { 2.5 } 
fn default_evap() -> f64 { 0.02 }
fn default_q0() -> f64 { 0.9 }
fn default_xi() -> f64 { 0.1 }
fn default_max_iterations() -> usize { 1000 }
fn default_time_limit() -> usize { std::usize::MAX }

