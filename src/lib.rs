use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool{ //thread pools own the sending side of the channel
    // threads : Vec<thread::JoinHandle<()>>, we created this but we dont want our raw threads to execute instantly, we want them to be in a queue and then executed by the threadpool 
    workers: Vec<Worker>, //here eache worker has metadata like id, thread and a queue of tasks
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    ///Create a new ThreadPool
    /// The size is the number of threads in Pool
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize)->ThreadPool{
        assert!(size>0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size); //we used this instead of Vec::new() because we want to preallocate the memory of 4.

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciever))); 
        }
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

//by getting id and shared reciever it will create a worken 
impl Worker{
    fn new (id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn( move || {
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");

                job();

                println!("Worker {id} finished the job.");
            }
        });

        Worker{id,thread}
    }
}