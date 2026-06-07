use std::sync::mpsc;
use std::thread;

pub struct Pool {
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send>>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl Pool {
    pub fn new(workers: usize) -> Self {
        Self {
            sender: None,
            handles: vec![],
        }
    }

    pub fn submit<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn shutdown(mut self) {
    }
}

/// Runs `n` jobs on a pool with `workers` threads, returning `(index, output)` pairs sorted by index.
pub fn run_indexed_jobs(workers: usize, n: usize) -> Vec<(usize, i32)> {
    vec![]
}
