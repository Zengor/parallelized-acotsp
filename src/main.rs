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
    let f = File::open(out_name)?;
    let mut writer = BufWriter::new(f);
    let best = results.best_timestamped();
    write!(writer, "BEST FOUND: {}", best.result.length)?;
    write!(writer, "BEST TOUR: {:?}", best.result.tour)?;
    write!(writer, "Found during iteration {} at {:?}", best.iteration, best.timestamp)?;
    write!(writer, "==========================")?;
    for (i,t) in results.log.iter().enumerate() {
        write!(writer, "-----Iter {}, new_best: {}", i, t.is_new_best)?;
        write!(writer, "length: {}", t.result.length)?;
        write!(writer, "tour: {:?}", t.result.tour)?;
    }
    Ok(())
}

fn main() {
    let start_time = std::time::Instant::now();
    let input_file = "test.tsp";
    let instance_file = read_instance_file(input_file);
    println!("test");
    let parameters = AcoParameters::default();
    println!("input read, moving to execution");
    let results = run_aco(&instance_file.data, &parameters);
    print_log(results, &format!("{} - {:?}", input_file, start_time)).expect("failed writing log");
}
