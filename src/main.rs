mod aco;
mod util;
mod instance_data;
mod tsplibreader;

use crate::aco::{AcoParameters, run_aco};
use crate::tsplibreader::{read_instance_file};

fn main() {
    let instance_file = read_instance_file("test.tsp");
    let parameters = AcoParameters::default();
    let results = run_aco(&instance_file.data, &parameters);
    //do something with results
}
