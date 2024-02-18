use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<dyn FnBox + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });
        Worker { id, thread }
    }
}
