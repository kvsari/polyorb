//! Example program that'll render a tetrahedron (Not finished yet).
use log::info;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Running tetrahedron demo...");

    polyorb::run::<polyorb::tetrahedron::Scene>("Tetrahedron")
}
