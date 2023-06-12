use std::sync::{Mutex, Condvar, MutexGuard};

pub struct Buffer<T> {
    capacity: usize,
    data: Mutex<Vec<T>>,
    condvar: Condvar
}

pub struct WorkerCounter {
    counter: Mutex<u8>,
    condvar: Condvar
}

impl<T> Buffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self { 
            capacity,
            data: Mutex::new(Vec::with_capacity(capacity)),
            condvar: Condvar::new(),
        }
    }

    pub fn write(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        while self.is_full(&data) {
            data = self.condvar.wait(data).unwrap();
        }

        data.push(value);
        self.condvar.notify_all();
    }

    pub fn read(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while self.is_empty(&data) {
            data = self.condvar.wait(data).unwrap();
        }

        let task = data.pop().unwrap();
        self.condvar.notify_all();
        task
    }

    pub fn is_full(&self, data: &MutexGuard<Vec<T>>) -> bool {
        data.len() >= self.capacity
    }

    pub fn is_empty(&self, data: &MutexGuard<Vec<T>>) -> bool {
        data.is_empty()
    }
}

impl WorkerCounter {
    pub fn new() -> Self{
        Self { counter: Mutex::new(0), condvar: Condvar::new() }
    }

    pub fn increase(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        self.condvar.notify_all();
    }

    pub fn decrease(&self) {
        let mut counter = self.counter.lock().unwrap();
        while *counter == 0 {
            counter = self.condvar.wait(counter).unwrap();
        }
        *counter -= 1;
        self.condvar.notify_all();
    }

    pub fn wait_all_workers_idle(&self) {
        let mut counter = self.counter.lock().unwrap();
        while *counter > 0 {
            counter = self.condvar.wait(counter).unwrap();
        }
    }
}