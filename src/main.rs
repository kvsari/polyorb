//! Example program for rendering various polyhedrons

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rendering.");

    polyorb::run::<polyorb::cube_example::Example>("Simple Cube")
}
