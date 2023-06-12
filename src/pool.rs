use std::thread::{JoinHandle, self};

use crate::monitors::{Buffer, WorkerCounter};

struct ThreadPool {
    buffer: Buffer<u8>,
    worker_counter: WorkerCounter,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(buffer_size: usize, worker_size: usize) -> Self {
        let mut workers: Vec<JoinHandle<()>> = vec![];

        for _ in 0..worker_size {
            workers.push(thread::spawn(move || {
                loop {
                    
                }
            }));
        }

        Self {
            buffer: Buffer::new(buffer_size),
            worker_counter: WorkerCounter::new(),
            workers
        }
    }
}