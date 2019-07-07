//! Demonstrate rendering the platonic solids.

use log::info;

use polyorb::platonic_solid;
use polyorb::light::Light;
use polyorb::scene::Scene;
use polyorb::{shader, presentation};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Running platonic solid demo...");

    let light1 = Light::new(
        cgmath::Point3::new(7f32, -5f32, 10f32),
        wgpu::Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 },
        60.0,
        1.0..20.0,
    );
    let light2 = Light::new(
        cgmath::Point3::new(-5f32, 7f32, 10f32),
        wgpu::Color { r: 1.0, g: 0.5, b: 0.5, a: 1.0 },
        45.0,
        1.0..20.0,
    );
    
    let solid = platonic_solid::Tetrahedron::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Cube::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Octahedron::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Dodecahedron::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Icosahedron::new(1.0, [0.0, 1.0, 0.0]);
    
    let s_vert = shader::load_vert("tetrahedron.vert", "main")?;
    let s_frag = shader::load_frag("tetrahedron.frag", "main")?;
    let scene = Scene::new()
        .shaders(&s_vert, &s_frag)
        .add_light(light1)
        .add_light(light2)
        .geometry(solid);

    presentation::run("Platonic Solid", scene)?;

    Ok(())
}

