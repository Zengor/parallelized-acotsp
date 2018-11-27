use std::time;
use log::info;
use itertools::Itertools;

use crate::aco::AntResult;

pub enum OutputType {
    StdOut,
    File(String),
    StdAndFile(String)
}


pub fn setup_logger(output: OutputType) -> Result<(), fern::InitError> {
    let f = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}",
                message
            ))
        })
        .level(log::LevelFilter::Debug);

    use self::OutputType::*;    
    match output {
        StdOut => f.chain(std::io::stdout()),
        File(s) => f.chain(fern::log_file(s)?),
        StdAndFile(s) => f.chain(std::io::stdout()).chain(fern::log_file(s)?),                
    }.apply()?;
    Ok(())
}

pub fn log_start() {
    info!("---NEW RUN---");
}

pub fn log_new_iter() {
    info!("#NewIter");
}
    
pub fn log_new_best(ant: &AntResult) {
    info!("!NEW BEST! VALUE: {}\n→{}", ant.value, ant.tour.iter().format(" "));        
}
