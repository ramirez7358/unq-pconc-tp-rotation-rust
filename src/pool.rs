use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    monitors::Buffer,
    task::{PoisonPill, Task},
};

pub struct ThreadPool {
    buffer: Arc<Buffer<Box<dyn Task>>>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(buffer_size: usize, worker_size: usize) -> Self {
        let mut workers: Vec<JoinHandle<()>> = vec![];
        let buffer: Arc<Buffer<Box<dyn Task>>> = Arc::new(Buffer::new(buffer_size));

        for _ in 0..worker_size {
            let b = Arc::clone(&buffer);

            workers.push(thread::spawn(move || loop {
                let task = b.read();
                match task.run() {
                    Ok(_) => continue,
                    Err(_) => break,
                };
            }));
        }

        Self { buffer, workers }
    }

    pub fn add_task(&self, task: Box<dyn Task>) {
        self.buffer.write(task);
    }

    pub fn wait_for_workers_to_finish(&mut self) {
        let capacity = Arc::clone(&self.buffer).capacity;
        for _ in 0..capacity {
            self.buffer.write(Box::new(PoisonPill {}))
        }
        for worker in self.workers.drain(..) {
            if let Err(error) = worker.join() {
                eprintln!("Error joining worker: {:?}", error);
            }
        }
    }
}
