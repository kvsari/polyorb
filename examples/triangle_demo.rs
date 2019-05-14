//! Example program that'll render a triangle.
use log::info;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Running triangle demo...");

    polyorb::run::<polyorb::triangle_example::TriangleScene01>("Triangle Scene 01")
}
