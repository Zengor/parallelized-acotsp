mod aco;
mod instance_data;
mod timer;
mod tsplibreader;
mod util;
mod parameters_reader;

use crate::aco::ResultLog;
use crate::aco::{run_aco, AcoParameters};
use crate::tsplibreader::read_instance_file;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Result};

fn print_log(results: ResultLog, out_path: &str, file_name: &str) -> Result<()> {
    std::fs::create_dir_all(out_path).expect("failed at creating path");
    let out: PathBuf = [out_path, file_name].iter().collect();
    let f = File::create(out.as_path())?;
    let mut writer = BufWriter::new(f);
    let best = results.best_timestamped();
    writeln!(writer, "BEST FOUND: {}", best.result.length)?;
    writeln!(writer, "BEST TOUR: {:?}", best.result.tour.iter().map(|i| i+1).collect::<Vec<usize>>())?;
    writeln!(
        writer,
        "Found on iteration {} at {}s",
        best.iteration,
        best.timestamp.as_secs()
    )?;
    writeln!(writer, "==========================")?;
    for (i, t) in results.log.iter().enumerate() {
        writeln!(writer, "-----Iter {}, new_best: {}", i+1, t.is_new_best)?;
        writeln!(
            writer,
            "length: {} time {}s",
            t.result.length,
            t.timestamp.as_secs()
        )?;
        writeln!(writer, "tour: {:?}", t.result.tour.iter().map(|i| i+1).collect::<Vec<usize>>())?;
    }
    Ok(())
}

fn main() {
    //let input_file = "a280.tsp";
    //let instance_file = read_instance_file(input_file);
    //let mut parameters = AcoParameters::default();
    let run_descriptions = crate::parameters_reader::read_run_file("mmasrun.json");
    for description in run_descriptions {
        println!("STARTING NEW RUN");
        println!("reading input file {}", &description.data_file);
        timer::restart_timer();
        let instance_file = read_instance_file(&description.data_file);
        println!("read input in {}s", timer::elapsed().as_secs());
        println!("---- starting runs");
        for run in 1..=description.num_runs {            
            println!("run {} of {}", run, description.num_runs);
            timer::restart_timer();
            let results = run_aco(&instance_file.data, &description.parameters);
            println!("total elapsed time {}s", timer::elapsed().as_secs());
            let out_file = format!("{}_{}.txt", description.out_path.replace('/', "_"), run);
            println!("printing results to {}/{}", &description.out_path, &out_file);
            print_log(results, &description.out_path, &out_file).expect("failed writing log file");
            println!("-----")
        }
        println!("==================");
    }
    println!("execution finished");
}
