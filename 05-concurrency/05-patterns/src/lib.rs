use std::sync::mpsc;
use std::thread;

pub struct WorkerPool {
    sender: Option<mpsc::Sender<Job>>,
    handles: Vec<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl WorkerPool {
    pub fn new(size: usize) -> Self {
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

/// Runs `n` jobs through a pool of `workers` threads, returning results in submission order.
pub fn run_jobs<F>(workers: usize, n: usize, f: F) -> Vec<i32>
where
    F: Fn(usize) -> i32 + Send + Sync + 'static,
{
    vec![]
}
