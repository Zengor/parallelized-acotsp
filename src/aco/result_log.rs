use super::ant::AntResult;

pub struct TimestampedResult {
    pub result: AntResult,
    pub iteration: usize,
    pub timestamp: std::time::Instant,
    pub is_new_best: bool
}

impl TimestampedResult {
    fn new(result: AntResult, iteration: usize, is_new_best: bool) -> Self {
        TimestampedResult {
            result,
            iteration,            
            timestamp: std::time::Instant::now(),
            is_new_best,
        }
    }

    fn value(&self) -> usize {
        self.result.value
    }
}

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

    pub fn best_value(&self) -> usize {
        self.log[self.best_so_far].value()
    }

    pub fn push(&mut self, new: AntResult, iteration: usize) {
        let is_new_best = new.value < self.best_value();
        let timestamped = TimestampedResult::new(new, iteration, is_new_best);
        self.log.push(timestamped);
        if is_new_best {
            self.best_so_far = self.log.len() - 1;
        }
    }
}
