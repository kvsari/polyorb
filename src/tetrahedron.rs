//! Tetrahedron. The first of the five platonic solids. It's a four faced polygon consisting
//! of equilateral triangles.
use std::{ops, mem};

use shaderc::ShaderKind;
use cgmath::{Matrix4, Vector3, Point3, Rad, Euler};

use crate::show::{Show, Camera, View, load_shader, common::*};
use crate::shape::{self, Vertex};

static deg60: cgmath::Deg<f32> = cgmath::Deg(60_f32);

struct Light {
    pos: Point3<f32>,
    colour: wgpu::Color,
    fov: f32,
    depth: ops::Range<f32>,
    //target_view: wgpu::TextureView,
}

#[derive(Clone, Copy)]
struct LightRaw {
    proj: [[f32; 4]; 4],
    pos: [f32; 4],
    colour: [f32; 4],
}

impl Light {
    fn to_raw(&self) -> LightRaw {
        use cgmath::{Deg, EuclideanSpace, Matrix4, PerspectiveFov, Point3, Vector3};

        let mx_view = Matrix4::look_at(self.pos, Point3::origin(), -Vector3::unit_z());
        let projection = PerspectiveFov {
            fovy: Deg(self.fov).into(),
            aspect: 1.0,
            near: self.depth.start,
            far: self.depth.end,
        };
        let mx_view_proj = cgmath::Matrix4::from(projection.to_perspective()) * mx_view;
        LightRaw {
            proj: *mx_view_proj.as_ref(),
            pos: [self.pos.x, self.pos.y, self.pos.z, 1.0],
            colour: [self.colour.r, self.colour.g, self.colour.b, 1.0],
        }
    }
}

pub struct Scene {
    lights: Vec<Light>,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    light_buf: wgpu::Buffer,
    projection_buf: wgpu::Buffer,
    rotation_buf: wgpu::Buffer,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_len: usize,
    camera: Camera<f32>,
    x_rotation: Rad<f32>,
    y_rotation: Rad<f32>,
}

impl Scene {
    fn new(
        lights: Vec<Light>,
        bind_group: wgpu::BindGroup,
        pipeline: wgpu::RenderPipeline,
        light_buf: wgpu::Buffer,
        projection_buf: wgpu::Buffer,
        rotation_buf: wgpu::Buffer,
        vertex_buf: wgpu::Buffer,
        index_buf: wgpu::Buffer,
        index_len: usize,
        camera: Camera<f32>,
        x_rotation: Rad<f32>,
        y_rotation: Rad<f32>,
    ) -> Self {
        Scene {
            lights,
            bind_group,
            pipeline,
            light_buf,
            projection_buf,
            rotation_buf,
            vertex_buf,
            index_buf,
            index_len,
            camera,
            x_rotation,
            y_rotation,
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
        let projection_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(p_ref);

        // Add rotation uniform buffer here (like the projection uniform buffer)
        let rotation = Matrix4::from_angle_y(Rad(0f32));
        let r_ref: &[f32; 16] = rotation.as_ref();
        let rotation_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
            )
            .fill_from_slice(r_ref);
                
        //let (vertexes, indexes) = shape::cube_01([0.0, 1.0, 0.0]);
        //let (vertexes, indexes) = shape::tetrahedron(1f32, [0.0, 1.0, 0.0]);
        //let (vertexes, indexes) = shape::octahedron(1f32, [0.0, 1.0, 0.0]);
        let (vertexes, indexes) = shape::icosahedron(1f32, [0.0, 1.0, 0.0]);
        
        let vertex_buf = device
            .create_buffer_mapped(vertexes.len(), wgpu::BufferUsageFlags::VERTEX)
            .fill_from_slice(&vertexes);

        let index_buf = device
            .create_buffer_mapped(indexes.len(), wgpu::BufferUsageFlags::INDEX)
            .fill_from_slice(&indexes);

         let lights = vec![
            Light {
                pos: cgmath::Point3::new(7f32, -5f32, 10f32),
                colour: wgpu::Color {
                    r: 0.5,
                    g: 1.0,
                    b: 0.5,
                    a: 1.0,
                },
                fov: 60.0,
                depth: 1.0..20.0,
                //target_view: shadow_target_views[0].take().unwrap(),
            },
            Light {
                pos: cgmath::Point3::new(-5f32, 7f32, 10f32),
                colour: wgpu::Color {
                    r: 1.0,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                },
                fov: 45.0,
                depth: 1.0..20.0,
                //target_view: shadow_target_views[1].take().unwrap(),
            },
        ];
        let light_buf_size = (2 * mem::size_of::<LightRaw>()) as u32;
        let mut light_buf_builder = device.create_buffer_mapped(
            2,
            wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
        );
        light_buf_builder.data[0] = lights[0].to_raw();
        light_buf_builder.data[1] = lights[1].to_raw();
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
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: wgpu::PipelineStageDescriptor {
                module: &fs_module,
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
        device.get_queue().submit(&[cmd_buf]);
        Scene::new(
            lights,
            bind_group,
            pipeline,
            light_buf,
            projection_buf,
            rotation_buf,
            vertex_buf,
            index_buf,
            indexes.len(),
            camera,
            Rad(0f32),
            Rad(0f32),
        )
    }

    fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) { }
    
    fn update(
        &mut self, camera_movement: Vector3<f32>, x_rot_inc: Rad<f32>, y_rot_inc: Rad<f32>,
    ) -> (&View<f32>, &Rad<f32>, &Rad<f32>) {
        self.x_rotation += x_rot_inc;
        self.y_rotation += y_rot_inc;
        (self.camera.move_camera(camera_movement), &self.x_rotation, &self.y_rotation)
    }
    
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder = device.create_command_encoder(&command_encoder_descriptor);

        // Use our latest projection even if the camera(eye) didn't change.
        {
            let projection = self.camera.projection();
            let p_ref: &[f32; 16] = projection.as_ref();
            let new_projection_buf = device
                .create_buffer_mapped(
                    16,
                    wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
                )
                .fill_from_slice(p_ref);
            
            encoder.copy_buffer_to_buffer(
                &new_projection_buf, 0, &self.projection_buf, 0, 16 * 4
            );
        }

        // Ditto with the rotation
        {
            let rotation = Matrix4::from(Euler::new(
                self.x_rotation, self.y_rotation, Rad(0f32)
            ));
            let r_ref: &[f32; 16] = rotation.as_ref();
            let new_rotation_buf = device
                .create_buffer_mapped(
                    16,
                    wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
                )
                .fill_from_slice(r_ref);

            encoder.copy_buffer_to_buffer(
                &new_rotation_buf, 0, &self.rotation_buf, 0, 16 * 4
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

