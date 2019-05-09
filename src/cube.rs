//! Data for cube.
use std::mem;

use crate::space::Vertex;

const static command_encoder_descriptor = wgpu::CommandEncoderDescriptor { todo: 0 };

pub fn vertexize(pos: [i8; 3]) -> Vertex {
    Vertex::new([pos[0] as f32, pos[1] as f32, pos[2] as f32, 1_f32])
}

pub fn create() -> (Vec<Vertex>, Vec<u16>) {
    let vertexes = [
        // Top
        vertexize([-1, -1, 1]),
        vertexize([1, -1, 1]),
        vertexize([1, 1, 1]),
        vertexize([-1, 1, 1]),
        
        // Bottom
        vertexize([-1, 1, -1]),
        vertexize([1, 1, -1]),
        vertexize([1, -1, -1]),
        vertexize([-1, -1, -1]),
        
        // Right
        vertexize([1, -1, -1]),
        vertexize([1, 1, -1]),
        vertexize([1, 1, 1]),
        vertexize([1, -1, 1]),
        
        // Left
        vertexize([-1, -1, 1]),
        vertexize([-1, 1, 1]),
        vertexize([-1, 1, -1]),
        vertexize([-1, -1, -1]),
        
        // Front
        vertexize([1, 1, -1]),
        vertexize([-1, 1, -1]),
        vertexize([-1, 1, 1]),
        vertexize([1, 1, 1]),
        
        // Back
        vertexize([1, -1, 1]),
        vertexize([-1, -1, 1]),
        vertexize([-1, -1, -1]),
        vertexize([1, -1, -1]),
    ];

    let indexes: &[u16] = &[
        0, 1, 2, 2, 3, 0,       // Top
        4, 5, 6, 6, 7, 4,       // Bottom
        8, 9, 10, 10, 11, 8,    // Right
        12, 13, 14, 14, 15, 12, // Left
        16, 17, 18, 18, 19, 16, // Front
        20, 21, 22, 22, 23, 20, // Back
    ];

    (vertexes.to_vec(), indexes.to_vec())
}

pub struct SingleCubeScene {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    uniform_buf: wgpu::Buffer,
    index_count: usize,
    bing_group: wgpu::BindGroup,
}

impl super::Scene for SingleCubeScene {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        let mut decoder = device.create_command_encoder(&command_encoder_descriptor);
        let (vertex_data, index_data) = create();
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsageFlags::VERTEX)
            .fill_from_slice(vertex_data.as_slice());

        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsageFlags::INDEX)
            .fill_from_slice(index_data.as_slice());

        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bg_layout],
            }
        );

        
        
    }
    // resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(&mut self, event: wgpu::winit::WindowEvent) { }
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) { }
}
