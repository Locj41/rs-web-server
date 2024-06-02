use std::{thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}};

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending teminate message to all worker");
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Worker {} is shutting down...", worker.thread_id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
impl ThreadPool {
    pub fn new(num_thread:usize) -> ThreadPool{
        assert!(num_thread > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(num_thread);
        
        let receiver = Arc::new(Mutex::new(receiver));


        for id in 0..num_thread {
            //create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers , sender}
    }

    pub fn excute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }
}

//worker to hold thread --> in order to thread will not run after spawn
struct Worker{
    thread_id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(thread_id:usize,
            receiver:Arc<Mutex<Receiver<Message>>>
           ) -> Worker 
    {
        let thread = thread::spawn( move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job ; excuting", thread_id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} got to  terminated", thread_id);
                    break;
                }
            }
        });
        Worker { thread_id, thread: Some(thread)}
    }
}


enum Message{
    NewJob(Job),
    Terminate,
}
