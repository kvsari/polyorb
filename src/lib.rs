//! # Polyorb
//!
//! Render various Goldberg polyhedrons.

use log::info;
use wgpu::winit::{self, Event};

//pub mod cube;
//pub mod space;

pub mod cube_example;

/// Platonic solid that is used as a base for the `GoldberPolyhedron`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlatonicSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

/// Fully contained scene description.
pub trait Scene {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self;
    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(&mut self, event: wgpu::winit::WindowEvent);
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device);
}

/// Taken heavily from the examples in wgpu crate. I have no idea otherwise how to use.
pub fn run<S: Scene>(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing the renderer.");
    
    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
    });

    info!("Setting up the window.");
    let mut event_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&event_loop)?;
    window.set_title(title);
    let w_size = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor());

    let surface = instance.create_surface(&window);
    let mut desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: w_size.width.round() as u32,
        height: w_size.height.round() as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &desc);

    info!("Initializing the scene.");
    let mut scene = S::init(&desc, &mut device);

    info!("Entering event loop.");
    let mut running = true;
    while running {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                winit::WindowEvent::KeyboardInput {
                    input: winit::KeyboardInput {
                        virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                        state: winit::ElementState::Pressed,
                        ..
                    },
                    ..
                }
                | winit::WindowEvent::CloseRequested => {
                    running = false;
                }
                _ => {
                    scene.update(event);
                }                
            },
            _ => (),
        });

        let frame = swap_chain.get_next_texture();
        scene.render(&frame, &mut device);
    }
    
    Ok(())
}

/*
/// The main data structure of the library.
pub struct GoldbergPolyhedron {
    base: PlatonicSolid,
    
}
*/
