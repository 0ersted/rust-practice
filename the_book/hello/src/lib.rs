use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    worker: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)();
    } 
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} gets a job", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} is being terminated", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id : id,
            worker : Some(thread),
        }
    }
}

impl ThreadPool {
    /**
     * Create a new ThreadPool
     *
     * size is the number of threads in the pool
     *
     * # Panics
     * will panic if the size is zero
     */
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, func: F) where F: FnOnce() + Send + 'static
    {
        let job = Box::new(func);
        
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination signal");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap(); 
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.worker.take() {
            println!("Worker {} is down", worker.id);
                thread.join().unwrap();
            }
        }
    }
}
