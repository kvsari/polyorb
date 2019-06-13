//! Scene traits and running
use std::{path, fs};

use log::{trace, info};
use cgmath::{Deg, Rad, Matrix4, Point3, Vector3, BaseFloat};
use wgpu::winit::{self, Event};
use shaderc::ShaderKind;

use crate::input;

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

#[derive(Debug, Copy, Clone)]
pub struct Perspective<S: BaseFloat> {
    fov: Rad<S>,
    aspect_ratio: S,
    near: S,
    far: S,
}

impl<S: BaseFloat>  Perspective<S> {
    pub fn new<T: Into<Rad<S>>>(fov: T, aspect_ratio: S, near: S, far: S) -> Self {
        Perspective { fov: fov.into(), aspect_ratio, near, far }
    }

    pub fn as_matrix(&self) -> Matrix4<S> {
        cgmath::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct View<S: BaseFloat> {
    from: Point3<S>,
    at: Point3<S>,
    up: Vector3<S>,
}

impl<S: BaseFloat> View<S> {
    pub fn new(from: Point3<S>, at: Point3<S>, up: Vector3<S>) -> Self {
        View { from, at, up }
    }

    pub fn as_matrix(&self) -> Matrix4<S> {
        cgmath::Matrix4::look_at(self.from, self.at, self.up)
    }

    pub fn move_camera(&mut self, increment: Vector3<S>) {
        self.from += increment;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Camera<S: BaseFloat> {
    perspective: Perspective<S>,
    view: View<S>,
}

impl<S: BaseFloat> Camera<S> {
    pub fn new(perspective: Perspective<S>, view: View<S>) -> Self {
        Camera { perspective, view }
    }

    pub fn projection(&self) -> Matrix4<S> {
        self.perspective.as_matrix() * self.view.as_matrix()
    }

    /// Move the camera position by the supplied increment and return a ref to the view.
    pub fn move_camera(&mut self, increment: Vector3<S>) -> &View<S> {
        self.view.move_camera(increment);
        &self.view
    }
}

/// Fully contained scene description.
pub trait Show {
    fn init(
        desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device, camera: Camera<f32>,
    ) -> Self;
    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(
        &mut self, camera_movement: Vector3<f32>, y_rot_inc: Rad<f32>
    ) -> (&View<f32>, &Rad<f32>);
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
    let w_width = w_size.width.round() as f32;
    let w_height = w_size.height.round() as f32;

    let perspective = Perspective::new(Deg(45f32), w_width / w_height, 1f32, 10f32);
    let view = View::new(
        Point3::new(0f32, -1f32, -1f32), Point3::new(0f32, 0f32, 0f32), -Vector3::unit_z()
    );
    let camera = Camera::new(perspective, view);
    let bindings = input::Bindings::default();
    let mut act_state: u16 = 0;

    let surface = instance.create_surface(&window);
    let mut desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: w_width as u32,
        height: w_height as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &desc);

    info!("Initializing the scene.");
    let mut scene = S::init(&desc, &mut device, camera);

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
                },
                winit::WindowEvent::KeyboardInput { input: keyboard_input, .. } => {
                    let maybie = input::handle_keyboard(
                        &keyboard_input, &bindings, &mut act_state
                    );
                    if let Some((camera_movement, rot_y)) = maybie {
                        let (view, tot_rot) = scene.update(camera_movement, rot_y);
                        trace!("{:?} && {:?}", view, tot_rot);
                    }
                },
                _ => (),
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
