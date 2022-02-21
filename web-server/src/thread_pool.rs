use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("executing on thread: {}", id);

            job();
        });

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);

            threads.push(Worker::new(id, receiver));
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<T>(&self, job: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        self.sender.send(job).unwrap();
    }
}
