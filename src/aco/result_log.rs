use super::ant::Ant;
use std::time::Duration;

#[derive(Debug)]
pub struct TimestampedResult {
    pub result: Ant,
    pub iteration: usize,
    pub timestamp: Duration,
    pub is_new_best: bool,
}

impl TimestampedResult {
    fn new(result: Ant, iteration: usize, is_new_best: bool) -> Self {
        TimestampedResult {
            result,
            iteration,
            timestamp: crate::timer::elapsed(),
            is_new_best,
        }
    }

    fn length(&self) -> u32 {
        self.result.length
    }
}

#[derive(Debug)]
pub struct ResultLog {
    pub log: Vec<TimestampedResult>,
    pub best_so_far: usize,
}

impl ResultLog {
    pub fn new(max_iters: usize) -> Self {
        ResultLog {
            log: Vec::with_capacity(max_iters),
            best_so_far: 0,
        }
    }

    pub fn latest_tour<'a>(&'a self) -> &'a Ant {
        &self.log[self.log.len() - 1].result
    }

    pub fn best_tour<'a>(&'a self) -> &'a Ant {
        &self.log[self.best_so_far].result
    }

    pub fn best_timestamped<'a>(&'a self) -> &'a TimestampedResult {
        &self.log[self.best_so_far]
    }

    pub fn best_length(&self) -> u32 {
        self.log[self.best_so_far].length()
    }

    pub fn push(&mut self, new: Ant, iteration: usize) {
        let is_new_best = self.log.is_empty() || new.length < self.best_length();
        let timestamped = TimestampedResult::new(new, iteration, is_new_best);
        self.log.push(timestamped);
        if is_new_best {
            self.best_so_far = self.log.len() - 1;
        }
    }
}
