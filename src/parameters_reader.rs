use serde_derive::Deserialize;
use std::fs::read_to_string;
use crate::aco::AcoParameters;

#[derive(Deserialize)]
pub struct RunDescription {
    #[serde(default = "default_num_runs")]
    num_runs: usize,
    data_file: String,
    parameters: AcoParameters,
}

pub read_run_file(f_name: &str) -> Vec<RunDescription> {
    let contents = read_to_string(f_name).expect("could not read parameter file");
    serde_json::from_str(&contents).expect("Could not convert paremeters from JSON")
}

fn default_num_runs() -> usize { 1 }