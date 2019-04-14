mod aco;
mod instance_data;
mod parameters_reader;
mod timer;
mod tsplibreader;
mod util;

use crate::aco::run_aco;
use crate::aco::ResultLog;
use crate::tsplibreader::read_instance_file;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

fn print_log(results: ResultLog, out_path: &str, file_name: &str, print_tour: bool) -> Result<()> {
    std::fs::create_dir_all(out_path).expect("failed at creating path");
    let out: PathBuf = [out_path, file_name].iter().collect();
    let f = File::create(out.as_path())?;
    let mut writer = BufWriter::new(f);
    let best = results.best_timestamped();
    writeln!(writer, "BEST FOUND: {}", best.result.length)?;
    writeln!(
        writer,
        "BEST TOUR: {:?}",
        best.result
            .tour
            .iter()
            .map(|i| i + 1)
            .collect::<Vec<usize>>()
    )?;

    writeln!(
        writer,
        "Found on iteration {} at {}.{}s",
        best.iteration,
        best.timestamp.as_secs(),
        best.timestamp.subsec_millis()
    )?;
    writeln!(writer, "==========================")?;
    for (i, t) in results.log.iter().enumerate() {
        writeln!(writer, "-----Iter {}, new_best: {}", i + 1, t.is_new_best)?;
        writeln!(
            writer,
            "length: {} time {}.{}s",
            t.result.length,
            t.timestamp.as_secs(),
            t.timestamp.subsec_millis()
        )?;
        if print_tour {
            writeln!(
                writer,
                "tour: {:?}",
                t.result.tour.iter().map(|i| i + 1).collect::<Vec<usize>>()
            )?;
        }
        //
    }
    Ok(())
}

fn main() {
    //let input_file = "a280.tsp";
    //let instance_file = read_instance_file(input_file);
    //let mut parameters = AcoParameters::default();
    let matches = App::new("Parallelized ACOTSP")
                            .version("0.1")
                            .author("Iago Almeida <ialmeida@edu.unifor.br>")
                            .about("Single core and Multicore implementations of MMAS and ACS metaheuristics for the TSP")
                            .arg(Arg::with_name("RUN DESCRIPTION FILE")
                                    .help("JSON file with the description of input files, parameters, number of runs, and algorithms to run")
                                    .required(true))
                            .arg(Arg::with_name(("Print Tour"))
                                    .short("t")
                                    .long("tour")
                                    .takes_value(false)
                                    .help("Whether the tour itself should be printed "))
                            .get_matches();

    let run_file_name = matches
        .value_of("RUN DESCRIPTION FILE")
        .expect("failed parsing argument");
    let run_descriptions = crate::parameters_reader::read_run_file(&run_file_name);
    for description in run_descriptions {
        println!("STARTING NEW RUN of {:?}", description.parameters.algorithm);
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
            let out_file = format!(
                "{}_{:?}_{}.txt",
                description.out_path.replace('/', "_"),
                description.parameters.algorithm,
                run
            );
            println!(
                "printing results to {}/{}",
                &description.out_path, &out_file
            );
            print_log(
                results,
                &description.out_path,
                &out_file,
                matches.is_present("Print Tour"),
            )
            .expect("failed writing log file");
            println!("-----")
        }
        println!("==================");
    }
    println!("execution finished");
}
