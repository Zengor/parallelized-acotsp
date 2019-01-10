mod aco;
mod util;
mod instance_data;
mod tsplibreader;
mod timer;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter,Result};
use crate::aco::{AcoParameters, run_aco};
use crate::tsplibreader::{read_instance_file};
use crate::aco::ResultLog;

fn print_log(results: ResultLog, out_name: &str) -> Result<()> {
    let f = File::create(out_name)?;
    let mut writer = BufWriter::new(f);
    let best = results.best_timestamped();
    writeln!(writer, "BEST FOUND: {}", best.result.length)?;
    writeln!(writer, "BEST TOUR: {:?}", best.result.tour)?;
    writeln!(writer, "Found on iteration {} at {}s", best.iteration, best.timestamp.as_secs())?;
    writeln!(writer, "==========================")?;
    for (i,t) in results.log.iter().enumerate() {
        writeln!(writer, "-----Iter {}, new_best: {}", i, t.is_new_best)?;
        writeln!(writer, "length: {} time {}s", t.result.length, t.timestamp.as_secs())?;
        writeln!(writer, "tour: {:?}", t.result.tour)?;
    }
    Ok(())
}

fn main() {
    let input_file = "a280.tsp";
    let instance_file = read_instance_file(input_file);
    let mut parameters = AcoParameters::default();
    parameters.algorithm = aco::Algorithm::MMASPar;
    parameters.num_ants = 280;
    println!("input read, moving to execution");
    let results = run_aco(&instance_file.data, &parameters);
    println!("execution finished, writing");
    print_log(results, &format!("{} - mmas par asdf4.txt", input_file)).expect("failed writing log");
}
