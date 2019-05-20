//! Triangle demo
use std::mem;

use shaderc::ShaderKind;

use crate::scene::{Scene, load_shader, common::*};

pub struct TriangleScene01 {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl Scene for TriangleScene01 {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        let vs_bytes = load_shader("triangle02.vert", "main", ShaderKind::Vertex).unwrap();
        let fs_bytes = load_shader("triangle02.frag", "main", ShaderKind::Fragment)
            .unwrap();

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[] }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            }
        );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: wgpu::PipelineStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            },
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: desc.format,
                color: wgpu::BlendDescriptor::REPLACE,
                alpha: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWriteFlags::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
        });

        TriangleScene01 { bind_group, pipeline }
    }
    
    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) { }
    fn update(&mut self, event: wgpu::winit::WindowEvent) { }
    
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder =
            device.create_command_encoder(&command_encoder_descriptor);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::GREEN,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group);
            rpass.draw(0..3, 0..1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    colour: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 2], colour: [f32; 3]) -> Self {
        Vertex { position, colour }
    }

    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}

fn generate_triangle_vertices() -> Vec<Vertex> {
    let vertex_data = [
        Vertex::new([0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, 0.5], [0.0, 1.0, 0.0]),
        Vertex::new([-0.5, 0.5], [0.0, 0.0, 1.0]),
    ];

    vertex_data.to_vec()
}

pub struct TriangleScene02 {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl Scene for TriangleScene02 {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        let vs_bytes = load_shader("triangle03.vert", "main", ShaderKind::Vertex).unwrap();
        let fs_bytes = load_shader("triangle03.frag", "main", ShaderKind::Fragment)
            .unwrap();

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let vertexes = generate_triangle_vertices();
        let vertex_buf = device
            .create_buffer_mapped(vertexes.len(), wgpu::BufferUsageFlags::VERTEX)
            .fill_from_slice(&vertexes);

        let bg_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStageFlags::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                }
            ]}
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bg_layout],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bg_layout,
            bindings: &[
                /*
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &vertex_buf,
                        range: 0..64,
                    }
                }
                */
            ],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: wgpu::PipelineStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            },
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: desc.format,
                color: wgpu::BlendDescriptor::REPLACE,
                alpha: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWriteFlags::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: Vertex::sizeof() as u32,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        attribute_index: 0,
                        format: wgpu::VertexFormat::Float2,
                        offset: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        attribute_index: 1,
                        format: wgpu::VertexFormat::Float3,
                        offset: 4 * 2,
                    },
                ],
            }],
            sample_count: 1,
        });

        TriangleScene02 { bind_group, pipeline }
    }
    
    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) { }
    fn update(&mut self, event: wgpu::winit::WindowEvent) { }
    
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
         let mut encoder =
            device.create_command_encoder(&command_encoder_descriptor);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group);
            rpass.draw(0..3, 0..1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}
