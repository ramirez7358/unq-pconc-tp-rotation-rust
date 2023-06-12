use std::sync::{Arc, Mutex};

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub trait Task: Send {
    fn run(&self) -> Result<(), &'static str>;
}

pub struct ShearTask {
    row: i32,
    start: i32,
    end: i32,
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
        row: i32,
        start: i32,
        end: i32,
        pivot_x: i32,
        pivot_y: i32,
        angle: f64,
        max_width: u32,
        max_height: u32,
        origin: Arc<Mutex<DynamicImage>>,
        destiny: Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
    ) -> Self {
        Self {
            row,
            start,
            end,
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
        for x in self.start..self.end {
            let x_tmp =
                (tan_half_angle * (self.row - self.pivot_y) as f64 + x as f64).round() as i32;
            let y_final =
                ((-sin_angle) * (x_tmp - self.pivot_x) as f64 + self.row as f64).round() as i32 - 1;
            let x_final = (tan_half_angle * (y_final - self.pivot_y) as f64 + x_tmp as f64).round()
                as i32
                + 1;

            if x_final >= 0
                && x_final < self.max_width as i32
                && y_final >= 0
                && y_final < self.max_height as i32
            {
                let pixel = origin.get_pixel(x as u32, self.row as u32).clone();
                destiny.put_pixel(x_final as u32, y_final as u32, pixel);
            }
        }
        Ok(())
    }
}

/*let x_tmp = ((self.angle / 2.0).tan() * (y - pivot_y) as f64 + x as f64).round() as i32;
let y_final =
    ((-self.angle.sin()) * (x_tmp - pivot_x) as f64 + y as f64).round() as i32 - 1;
let x_final = ((self.angle / 2.0).tan() * (y_final - pivot_y) as f64 + x_tmp as f64)
    .round() as i32
    + 1;

if x_final >= 0
    && x_final < self.max_width as i32
    && y_final >= 0
    && y_final < self.max_height as i32
{
    let mut destiny = self.destiny.lock().unwrap();
    let pixel = self.origin.get_pixel(x, self.row).clone();
    println!("x final {}, y final {}", x_final, y_final);
    destiny.put_pixel(x_final as u32, y_final as u32, pixel);
}*/
