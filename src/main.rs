use std::{sync::Arc, thread};

use monitors::Buffer;

pub mod monitors;
pub mod pool;

fn main() {
    let buffer = Arc::new(Buffer::new(1));
    let buffer_clone = Arc::clone(&buffer);
    let buffer_clone2 = Arc::clone(&buffer);

    let read_handle = thread::spawn(move || {
        loop {
            let number = buffer_clone2.read();
            println!("{}", number);
        }
    });

    let handle = thread::spawn(move || {
        buffer_clone.write(1);
        buffer_clone.write(2);
        buffer_clone.write(3);
        buffer_clone.write(4);
        buffer_clone.write(5);
    });
}
