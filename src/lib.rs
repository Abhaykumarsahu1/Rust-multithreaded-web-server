use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool{ //thread pools own the sending side of the channel
    // threads : Vec<thread::JoinHandle<()>>, we created this but we dont want our raw threads to execute instantly, we want them to be in a queue and then executed by the threadpool 
    workers: Vec<Worker>, //here eache worker has metadata like id, thread and a queue of tasks
    sender: Option<mpsc::Sender<Job>>, //we need Option for graceful shutdown coz we have to move sender out but we can't move it out while still its in use also it is a borrow inside drop method so we use option as it will return none if the sender is not there so the field will not emptied out. drop method will take care of the rest
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
        ThreadPool {workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){

        drop(self.sender.take());

        for mut worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

//by getting id and shared reciever it will create a worken 
impl Worker{
    fn new (id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn( move || {
            loop {
                //we are following this match pattern to handle the error like recv is waiting alright but when sender will be dropped then recv will not wait but will panick and return error so we have to handle that error
                let message = reciever.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                
                        job();
                    }

                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }

                }
            }
        });

        Worker{id,
            thread: Some(thread),
        }
    }
}