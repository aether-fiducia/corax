use std::thread;
use std::sync::{mpsc, Arc, Mutex};

/// Pool of threads that will execute closure passed to them in sequence
///
/// The pool's size has no default values and iis specified at the `new` function call
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// Internal structure to hold spawned threads will waiting for a closure to execute
struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize,
}

/// A job is a trait object that holds the type of the closure used in the ThreadPool's execute
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {

    /// Spawn a new worker given a UUID (in this case it is just a single unsigned number)
    ///
    /// Id's are stored as plain numbers with no hashing, so spawn called must be unique.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break; // Fuck safety amIright?
                }
            }
        });
        Worker { thread: Some(thread), id }
    }
}

impl ThreadPool {

    /// Spawn new ThreadPool of given size
    ///
    /// The parameter dictates the number of workers to be spawned into the ThreadPool
    ///
    /// # Panics
    ///
    /// The `new` function will panic on a negative input.
    pub fn new(size: usize) -> Self {
        // Crash on negative threadpool size
        assert!(size > 0);

        // Vec to hold spawned workers
        let mut workers = Vec::with_capacity(size);

        // Look at the safe ass communication
        let (sender, receiver) = mpsc::channel();

        // This is thread safety yes?
        let receiver = Arc::new(Mutex::new(receiver));

        // Make the little workers go dawg
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
    }

    // Equivalent to running a closure with thread::spawn
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        // Storing on the heap? Yes sir, that will totally work.
        // Yes this is of type job, check mate aetheists!
        let job = Box::new(f);

        // Error handling who?
        self.sender.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    /// Drop method should perform a graceful shutdown of all workers ini the pool
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            // If there is an active thread then change the Option to None and join the underlying thread
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
