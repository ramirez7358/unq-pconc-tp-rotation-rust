use std::sync::{Condvar, Mutex, MutexGuard};

pub struct Buffer<T> {
    pub capacity: usize,
    data: Mutex<Vec<T>>,
    condvar: Condvar,
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
        self.condvar.notify_one();
    }

    pub fn read(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while self.is_empty(&data) {
            data = self.condvar.wait(data).unwrap();
        }

        let task = data.remove(0);
        self.condvar.notify_one();
        task
    }

    pub fn is_full(&self, data: &MutexGuard<Vec<T>>) -> bool {
        data.len() >= self.capacity
    }

    pub fn is_empty(&self, data: &MutexGuard<Vec<T>>) -> bool {
        data.is_empty()
    }
}
