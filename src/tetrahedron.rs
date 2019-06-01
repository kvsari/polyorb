//! Tetrahedron. The first of the five platonic solids. It's a four faced polygon consisting
//! of equilateral triangles.
use std::mem;

use shaderc::ShaderKind;
use cgmath::{Point2, Basis2, Rotation, Rotation2, Vector3};
use wgpu::winit::{WindowEvent, KeyboardInput};

use crate::show::{Show, Camera, View, load_shader, common::*};
use crate::shape::{square, equilateral_triangle};

static deg60: cgmath::Deg<f32> = cgmath::Deg(60_f32);

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], colour: [f32; 3]) -> Self {
        Vertex { position, colour }
    }

    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}

fn gen_shape_01(side_len: f32, colour: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let (points, index) = equilateral_triangle(side_len);
    let vertexes = points
        .into_iter()
        .map(|p| Vertex::new([p.x, p.y, 0_f32], colour))
        .collect();
    
    (vertexes, index.to_vec())
}

fn gen_shape_02(side_len: f32, colour: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let (points, index) = square(side_len);
    let vertexes = points
        .into_iter()
        .map(|p| Vertex::new([p.x, p.y, 0_f32], colour))
        .collect();

    (vertexes, index.to_vec())
}

pub struct Scene {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    uniform_buf: wgpu::Buffer,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,    
    index_len: usize,
    camera: Camera<f32>,
}

impl Scene {
    fn new(
        bind_group: wgpu::BindGroup,
        pipeline: wgpu::RenderPipeline,
        uniform_buf: wgpu::Buffer,
        vertex_buf: wgpu::Buffer,
        index_buf: wgpu::Buffer,
        index_len: usize,
        camera: Camera<f32>,
    ) -> Self {
        Scene {
            bind_group, pipeline, uniform_buf, vertex_buf, index_buf, index_len, camera,
        }
    }
}

impl Show for Scene {
    fn init(
        desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device, camera: Camera<f32>,
    ) -> Self {
        let mut cmd_encoder = device.create_command_encoder(&command_encoder_descriptor);
        
        let vs_bytes = load_shader("tetrahedron.vert", "main", ShaderKind::Vertex).unwrap();
        let fs_bytes = load_shader("tetrahedron.frag", "main", ShaderKind::Fragment)
            .unwrap();

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let projection = camera.projection();
        let p_ref: &[f32; 16] = projection.as_ref();
        let uniform_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(p_ref);

        let (vertexes, indexes) = gen_shape_01(1f32, [1.0, 0.0, 0.0]);
        let vertex_buf = device
            .create_buffer_mapped(vertexes.len(), wgpu::BufferUsageFlags::VERTEX)
            .fill_from_slice(&vertexes);

        let index_buf = device
            .create_buffer_mapped(indexes.len(), wgpu::BufferUsageFlags::INDEX)
            .fill_from_slice(&indexes);

        let bg_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStageFlags::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                }
            ]}            
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { bind_group_layouts: &[&bg_layout], }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bg_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        //buffer: &vertex_buf,
                        buffer: &uniform_buf,
                        //range: 0..18,
                        range: 0..64,
                    }
                }
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
                        format: wgpu::VertexFormat::Float3,
                        offset: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        attribute_index: 1,
                        format: wgpu::VertexFormat::Float3,
                        offset: 4 * 3,
                    },
                ],
            }],
            sample_count: 1,
        });

        let cmd_buf = cmd_encoder.finish();
        device.get_queue().submit(&[cmd_buf]);
        Scene::new(
            bind_group, pipeline, uniform_buf, vertex_buf, index_buf, indexes.len(), camera,
        )
    }

    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) { }
    
    fn update(&mut self, camera_movement: Vector3<f32>) -> &View<f32> {
        self.camera.move_camera(camera_movement)
    }
    
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder = device.create_command_encoder(&command_encoder_descriptor);

        // Use our latest projection even if the camera(eye) didn't change.
        {
            let projection = self.camera.projection();
            let p_ref: &[f32; 16] = projection.as_ref();
            let new_uniform_buf = device
                .create_buffer_mapped(
                    16,
                    wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
                )
                .fill_from_slice(p_ref);
            
            encoder.copy_buffer_to_buffer(
                &new_uniform_buf, 0, &self.uniform_buf, 0, 16 * 4
            );
        }

        // Render
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLUE,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group);
            rpass.set_index_buffer(&self.index_buf, 0);
            rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
            rpass.draw_indexed(0..self.index_len as u32, 0, 0..1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}

