use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use image::ImageBuffer;

use crate::{pool::ThreadPool, task::ShearTask};

const REGION_SIZE: i32 = 100;

pub struct Rotator {
    image_path: String,
}

impl Rotator {
    pub fn new(image_path: String) -> Self {
        Self { image_path }
    }

    pub fn run(&self) {
        let origin = image::open(self.image_path.as_str()).unwrap();
        let destiny = Arc::new(Mutex::new(ImageBuffer::new(
            origin.width(),
            origin.height(),
        )));

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut thread_pool = ThreadPool::new(2, 2);

        let regions_per_row = (origin.width() as f32 / REGION_SIZE as f32).ceil() as i32;

        for row in 0..origin.height() {
            for x in 0..regions_per_row {
                let start = x * REGION_SIZE;
                let end = (start + REGION_SIZE).min(origin.width() as i32);
                let task = ShearTask::new(
                    row as i32,
                    start,
                    end,
                    (origin.width() / 2) as i32,  // hardcode
                    (origin.height() / 2) as i32, // hardcode
                    (45 as f64).to_radians(),     // hardcode
                    origin.width(),
                    origin.height(),
                    origin.clone(),
                    Arc::clone(&destiny),
                );
                thread_pool.add_task(Box::new(task));
            }
        }

        thread_pool.wait_for_workers_to_finish();
        destiny.lock().unwrap().save("./output.jpg").unwrap();

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");

        println!("Time elapsed: {} ms", end.as_millis() - start.as_millis());
    }
}
