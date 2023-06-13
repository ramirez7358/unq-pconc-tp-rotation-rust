use std::sync::{Arc, Mutex};

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub trait Task: Send {
    fn run(&self) -> Result<(), &'static str>;
}

pub struct ShearTask {
    start_y: i32,
    end_y: i32,
    start_x: i32,
    end_x: i32,
    pivot_x: i32,
    pivot_y: i32,
    angle: f64,
    max_width: u32,
    max_height: u32,
    origin: Arc<Mutex<DynamicImage>>,
    destiny: Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
}
pub struct PoisonPill;

impl ShearTask {
    pub fn new(
        start_y: i32,
        end_y: i32,
        start_x: i32,
        end_x: i32,
        pivot_x: i32,
        pivot_y: i32,
        angle: f64,
        max_width: u32,
        max_height: u32,
        origin: Arc<Mutex<DynamicImage>>,
        destiny: Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
    ) -> Self {
        Self {
            start_y,
            end_y,
            start_x,
            end_x,
            pivot_x,
            pivot_y,
            angle,
            max_width,
            max_height,
            origin,
            destiny,
        }
    }
}

impl Task for PoisonPill {
    fn run(&self) -> Result<(), &'static str> {
        Err("")
    }
}

impl Task for ShearTask {
    fn run(&self) -> Result<(), &'static str> {
        let sin_angle = self.angle.sin();
        let tan_half_angle = (self.angle / 2.0).tan();
        let mut destiny = self.destiny.lock().unwrap();
        let origin = self.origin.lock().unwrap();

        for y in self.start_y..self.end_y {
            for x in self.start_x..self.end_x {
                let x_tmp = (tan_half_angle * (y - self.pivot_y) as f64 + x as f64).round() as i32;

                let y_final =
                    ((-sin_angle) * (x_tmp - self.pivot_x) as f64 + y as f64).round() as i32 - 1;
                let x_final = (tan_half_angle * (y_final - self.pivot_y) as f64 + x_tmp as f64)
                    .round() as i32
                    + 1;

                if x_final >= 0
                    && x_final < self.max_width as i32
                    && y_final >= 0
                    && y_final < self.max_height as i32
                {
                    let pixel = origin.get_pixel(x as u32, y as u32).clone();
                    destiny.put_pixel(x_final as u32, y_final as u32, pixel);
                }
            }
        }

        Ok(())
    }
}
