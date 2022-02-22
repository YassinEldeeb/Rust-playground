use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

/// `Worker` is a wrapper around a `thread` to hold an `Id` and an `Option<JoinHandle<()>>`
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Instantiate a new `Worker` using an `id` and a receiver to receive sent messages
    /// which will execute the requests' handlers.
    ///
    /// Opens a new thread that loops forever and listenning for received messages to
    /// process responses for incoming requests
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = receiver
                .lock()
                .expect("Couldn't acquire the lock to receive the sent message!")
                .recv()
                .expect("Couldn't receive a message cause the Sender has disconnected!");

            match msg {
                Message::NewJob(job) => {
                    println!("executing on thread: {}", id);

                    job();
                }
                Message::Terminate => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// `Message` variants get sent through `mpsc` channel from the `ThreadPool`
/// to the working threads to indicate wether to process a particular `Job`
/// or prepare for shut down by exiting the loop to stop listening for incoming requests.
enum Message {
    NewJob(Job),
    Terminate,
}

/// A Thread Pool implementation that allows you to use `N` number of threads for processing a particular
/// job in parallel with other running jobs.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).expect("Couldn't send a Terminate message for the threads to stop listening for incoming requests' handlers!");
        }

        for worker in &mut self.workers {
            println!("Shutting down thread: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect(
                    "Couldn't join the thread to wait for the main thread till it finishes!",
                );
            }
        }
    }
}

impl ThreadPool {
    /// Instantiate a new `ThreadPool` struct given a `size` which indicates the number of threads
    /// to spawn in the pool.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);

            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }

    /// Takes a closure for the job to execute that then will be sent as a message through
    /// `mpsc` channel for one of the running threads in the pool to pick it up and process it.
    pub fn execute<T>(&self, job: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        self.sender
            .send(Message::NewJob(job))
            .expect("Couldn't send a new job closure cause receiving ends has stopped listening");
    }
}
