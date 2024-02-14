pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
};
use std::thread;


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

