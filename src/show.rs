//! Scene traits and running
use std::{path, fs};

use log::info;
use wgpu::winit::{self, Event};
use shaderc::ShaderKind;

pub fn load_shader(
    name: &str, entry: &str, kind: ShaderKind
) -> Result<Vec<u8>, shaderc::Error> {
    let mut compiler = shaderc::Compiler::new()
        .ok_or(shaderc::Error::NullResultObject("Can't create compiler.".to_owned()))?;

    let filepath = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("shaders")
        .join(name);

    let contents = fs::read_to_string(&filepath)
        .map_err(|e| shaderc::Error::NullResultObject(format!("{}", &e)))?;

    let artifact = compiler.compile_into_spirv(&contents, kind, name, entry, None)?;
    
    Ok(artifact.as_binary_u8().to_owned())
}

/// Fully contained scene description.
pub trait Show {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self;
    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(&mut self, event: wgpu::winit::WindowEvent);
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device);
}

/// Taken heavily from the examples in wgpu crate. I have no idea otherwise how to use.
pub fn run<S: Show>(title: &str) -> Result<(), Box<dyn std::error::Error>> {
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

pub mod common {
    
    pub static command_encoder_descriptor: wgpu::CommandEncoderDescriptor
        = wgpu::CommandEncoderDescriptor { todo: 0 };
}
