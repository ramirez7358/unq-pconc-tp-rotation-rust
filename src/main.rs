use crate::rotator::Rotator;

pub mod monitors;
pub mod pool;
pub mod rotator;
pub mod task;

fn main() {
    Rotator::new(String::from("./1024x1024.jpg")).run();
}
