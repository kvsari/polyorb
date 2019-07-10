//! Typestate that holds render pipelines, perspectives and assets.
use std::mem;

use derive_getters::Getters;
use num_traits::identities::Zero;
use cgmath::Matrix4;

use crate::presentation::{Initializable, Renderable};
use crate::light::{Light, LightRaw};

const MAX_LIGHTS: usize = 10;

/// Final vertex data ready for consumption by the video device. A vector of these will be
/// the last step in getting some arbitrary geometry loaded in video memory for rendering.
#[derive(Debug, Copy, Clone, Getters)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3], // Consider removing this in the upcoming refactor ?? really??
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }

    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}

/// Vertex data (triangles) and indexes and colours for slurping into video memory.
pub trait Geometry {
    fn geometry(&self) -> (Vec<Vertex>, Vec<u16>);
}

/// Begin construction of a new `Scene`.
pub struct Begin;

pub struct Lights {
    frag: Vec<u8>,
    vert: Vec<u8>,
    lights: Vec<Light>,
}

pub struct Prepare<T: Geometry> {
    frag: Vec<u8>,
    vert: Vec<u8>,
    lights: Vec<Light>,
    geometry: T,
}

pub struct Ready {
    light_buf: wgpu::Buffer,
    projection_buf: wgpu::Buffer,
    rotation_buf: wgpu::Buffer,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_len: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

/// Holds all pertinent data and configuration for rendering a scene onto the video device.
/// Uses the typestate pattern to ensure correct usage. This is not a game engine.
pub struct Scene<S> {
    state: S,
}

impl Scene<Begin> {
    pub fn new() -> Self {
        Scene { state: Begin }
    }
   
    pub fn shaders(self, vert: &[u8], frag: &[u8]) -> Scene<Lights> {
        Scene {
            state: Lights {
                frag: frag.to_owned(),
                vert: vert.to_owned(),
                lights: Vec::new(),
            }
        }
    }
}

impl Scene<Lights> {
    /// Add a light. Don't add more than `MAX_LIGHTS` as they'll be ignored. If no lights
    /// are added the shape won't be visible.
    ///
    /// TODO: Signal to the fragment shader the number of lights loaded.
    ///       Shader currently assumes exactly two.
    pub fn add_light(mut self, light: Light) -> Self {
        self.state.lights.push(light);
        self
    }

    pub fn geometry<T: Geometry>(self, geometry: T) -> Scene<Prepare<T>> {
        let p = Prepare {
            frag: self.state.frag,
            vert: self.state.vert,
            lights: self.state.lights,
            geometry,
        };

        Scene { state: p }
    }
}

impl<T: Geometry> Scene<Prepare<T>> {
    pub fn prepare(
        &self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
    ) -> Scene<Ready> {
        let cmd_encoder = device
            .create_command_encoder(
                &wgpu::CommandEncoderDescriptor { todo: 0 }
            );
        
        let m_vert = device.create_shader_module(&self.state.vert);
        let m_frag = device.create_shader_module(&self.state.frag);
       
        let projection = Matrix4::zero();
        let p_ref: &[f32; 16] = projection.as_ref();
        let projection_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(p_ref);

        // Add rotation uniform buffer here (like the projection uniform buffer)
        let rotation = Matrix4::zero();
        let r_ref: &[f32; 16] = rotation.as_ref();
        let rotation_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(r_ref);

        let (vertices, index) = self.state.geometry.geometry();
        
        let vertex_buf = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsageFlags::VERTEX)
            .fill_from_slice(&vertices);

        let index_buf = device
            .create_buffer_mapped(index.len(), wgpu::BufferUsageFlags::INDEX)
            .fill_from_slice(&index);

        // TODO: Make number of lights dynamic between 0 to `MAX_LIGHTS`.
        let light_buf_size = (2 * LightRaw::sizeof()) as u32;
        let light_buf_builder = device
            .create_buffer_mapped(
                2,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            );        
        light_buf_builder.data[0] = self.state.lights[0].to_raw();
        light_buf_builder.data[1] = self.state.lights[1].to_raw();
        let light_buf = light_buf_builder.finish();

        let bg_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[
                // Projection uniform buffer layout
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStageFlags::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                },
                
                // Rotation uniform buffer layout
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStageFlags::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                },
                
                // Lights
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStageFlags::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer,
                },
            ]}            
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { bind_group_layouts: &[&bg_layout], }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bg_layout,
            bindings: &[
                // Projection uniform buffer binding
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &projection_buf,
                        range: 0..64,
                    }
                },
                
                // Rotation uniform buffer binding
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &rotation_buf,
                        range: 0..64
                    }
                },
                
                // Light uniform buffer binding
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &light_buf,
                        range: 0..light_buf_size,
                    }
                },
            ],
        });
        
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &m_vert,
                entry_point: "main",
            },
            fragment_stage: wgpu::PipelineStageDescriptor {
                module: &m_frag,
                entry_point: "main",
            },
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 2,
                depth_bias_slope_scale: 2.0,
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
                    // These are the vertexes. Location 0.
                    wgpu::VertexAttributeDescriptor { 
                        attribute_index: 0,
                        format: wgpu::VertexFormat::Float3,
                        offset: 0,
                    },
                    
                    // Our per vertex normal. Location 1.
                    wgpu::VertexAttributeDescriptor {
                        attribute_index: 1,
                        format: wgpu::VertexFormat::Float3,
                        offset: 4 * 3,
                    },
                    
                    // This is the colour. Location 2.
                    wgpu::VertexAttributeDescriptor { 
                        attribute_index: 2,
                        format: wgpu::VertexFormat::Float3,
                        offset: 4 * 6,
                    },
                ],
            }],
            sample_count: 1,
        });
        
        let cmd_buf = cmd_encoder.finish();
        
        device.get_queue()
            .submit(&[cmd_buf]);

        let index_len = index.len();
        
        let ready = Ready {
            light_buf,
            projection_buf,
            rotation_buf,
            vertex_buf,
            index_buf,
            index_len,
            bind_group,
            pipeline,
        };

        Scene { state: ready }
    }
}

impl Renderable for Scene<Ready> {
    fn render(
        &mut self,
        projection: &Matrix4<f32>,
        rotation: &Matrix4<f32>,
        frame: &wgpu::SwapChainOutput,
        device: &mut wgpu::Device,
    ) {
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 }
        );

        // Update with the sent projection
        {
            let p_ref: &[f32; 16] = projection.as_ref();
            let new_projection_buf = device
                .create_buffer_mapped(
                    16,
                    wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
                )
                .fill_from_slice(p_ref);
            
            encoder.copy_buffer_to_buffer(
                &new_projection_buf, 0, &self.state.projection_buf, 0, 16 * 4
            );
        }

        // Ditto with the rotation
        {
            let r_ref: &[f32; 16] = rotation.as_ref();
            let new_rotation_buf = device
                .create_buffer_mapped(
                    16,
                    wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
                )
                .fill_from_slice(r_ref);

            encoder.copy_buffer_to_buffer(
                &new_rotation_buf, 0, &self.state.rotation_buf, 0, 16 * 4
            );
        }

        // Render
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
            rpass.set_pipeline(&self.state.pipeline);
            rpass.set_bind_group(0, &self.state.bind_group);
            rpass.set_index_buffer(&self.state.index_buf, 0);
            rpass.set_vertex_buffers(&[(&self.state.vertex_buf, 0)]);
            rpass.draw_indexed(0..self.state.index_len as u32, 0, 0..1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}

impl<T: Geometry> Initializable for Scene<Prepare<T>> {
    type Ready = Scene<Ready>;
    
    fn init(
        self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device
    ) -> Self::Ready {
        self.prepare(desc, device)
    }
}
