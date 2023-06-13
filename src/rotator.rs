use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use image::ImageBuffer;

use crate::{pool::ThreadPool, task::ShearTask};

const REGION_SIZE: i32 = 200;

pub struct Rotator {
    image_path: String,
}

impl Rotator {
    pub fn new(image_path: String) -> Self {
        Self { image_path }
    }

    pub fn run(&self) {
        let origin_arc: Arc<Mutex<image::DynamicImage>> =
            Arc::new(Mutex::new(image::open(self.image_path.as_str()).unwrap()));

        let width;
        let height;
        {
            let origin = origin_arc.lock().unwrap();
            width = origin.width();
            height = origin.height();
        }
        let destiny = Arc::new(Mutex::new(ImageBuffer::new(width, height)));

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut thread_pool = ThreadPool::new(64, 16);

        let regions_per_column = (height as f32 / REGION_SIZE as f32).ceil() as i32;
        let regions_per_row = (width as f32 / REGION_SIZE as f32).ceil() as i32;

        for y in 0..regions_per_column {
            for x in 0..regions_per_row {
                let start_x = x * REGION_SIZE;
                let end_x = (start_x + REGION_SIZE).min(width as i32);

                let start_y = y * REGION_SIZE;
                let end_y = (start_y + REGION_SIZE).min(height as i32);

                let task = ShearTask::new(
                    start_y,
                    end_y,
                    start_x,
                    end_x,
                    (width / 2) as i32,        // hardcode
                    (height / 2) as i32,       // hardcode
                    (-45 as f64).to_radians(), // hardcode
                    width,
                    height,
                    Arc::clone(&origin_arc),
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
