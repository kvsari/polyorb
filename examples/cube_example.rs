//! Example program that'll render a cube from the library example.
use log::info;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Running library cube example.");

    polyorb::run::<polyorb::cube_example::Example>("Cube Example")
}
