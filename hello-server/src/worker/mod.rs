use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// # Panics
    ///
    /// Will panic if the receiver is poisoned.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            if let Ok(job) = message {
                println!("Worker {id} got a job; executing.");
                job();
            } else {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
        }));
        Worker { id, thread }
    }
}
