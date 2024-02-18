use crate::worker::{Job, Worker};
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // todo:  -> Result<ThreadPool, PoolCreationError>
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<Worker>>();
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
