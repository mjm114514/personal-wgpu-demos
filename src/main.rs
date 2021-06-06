use core::time;

use cgmath::{Deg, PerspectiveFov};
use controller::Controller;
use timer::Timer;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use futures::executor::block_on;
use camera::Camera;

mod texture;
mod camera;
mod controller;
mod timer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    // Front face
    Vertex{ position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [-1.0,  1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [ 1.0,  1.0, -1.0], tex_coords: [1.0, 0.0] },
    Vertex{ position: [ 1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    // back face
    Vertex{ position: [-1.0, -1.0,  1.0], tex_coords: [1.0, 1.0] },
    Vertex{ position: [ 1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [ 1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [-1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    // top face
    Vertex{ position: [-1.0,  1.0, -1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [-1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [ 1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    Vertex{ position: [ 1.0,  1.0, -1.0], tex_coords: [1.0, 1.0] },
    // bottom face
    Vertex{ position: [-1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    Vertex{ position: [ 1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [ 1.0, -1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [-1.0, -1.0,  1.0], tex_coords: [1.0, 0.0] },
    // left face
    Vertex{ position: [-1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [-1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [-1.0,  1.0, -1.0], tex_coords: [1.0, 0.0] },
    Vertex{ position: [-1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    // right face
    Vertex{ position: [ 1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
    Vertex{ position: [ 1.0,  1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex{ position: [ 1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    Vertex{ position: [ 1.0, -1.0,  1.0], tex_coords: [1.0, 1.0] },
];

const INDICES: &[u16] = &[
    // front face
    0, 1, 2,
    0, 2, 3,
    // back face
    4, 5, 6,
    4, 6, 7,
    // left face
    8, 9, 10,
    8, 10, 11,
    // right face
    12, 13, 14,
    12, 14, 15,
    // top face
    16, 17, 18,
    16, 18, 19,
    // bottom face
    20, 21, 22,
    20, 22, 23,
];

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.get_view_proj().into();
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>, 

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,

    uniforms: Uniforms,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,

    camera: Camera,
    controller: Controller,
    timer: Timer,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU.
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface)
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None, // Trace path
        ).await.unwrap();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
            },
        );

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("diffuse_bind_group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ]
            }
        );

        let camera = Camera::new(swap_chain_desc.width as f32 / swap_chain_desc.height as f32);

        println!("{:?}", camera.projection_matrix);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ]
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[
                    Vertex::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;

        let controller = Controller::new(10.0);
        let timer = Timer::new();

        Self {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            size,

            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,

            uniforms,
            uniform_bind_group,
            uniform_buffer,

            camera,
            controller,
            timer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.swap_chain_desc.width = new_size.width;
        self.swap_chain_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);

        self.camera.set_lens(PerspectiveFov {
            aspect: self.swap_chain_desc.width as f32 / self.swap_chain_desc.height as f32,
            far: 0.1,
            near: 100.0,
            fovy: Deg(45.0).into(),
        });
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.controller.process_events(event)
    }

    fn update(&mut self) {
        self.timer.tick();
        self.controller.update_all(&mut [&mut self.camera], self.timer.delta_time());
        println!("{:?}", self.camera.right());
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(State::new(&window));

    let mut frame_count = 0u32;
    let mut time_elapsed = 0f32;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id
        } if window_id == window.id() => if !state.input(event) {
            match event {

                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },

                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                },

                _ => {}
            }
        },

        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {},
                // Recreate the swap_chain if lost.
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit.
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame.
                Err(e) => eprint!("{:?}", e),
            }
            frame_count += 1;
        },

        Event::MainEventsCleared => {
            // RedrawRequest will only trigger once, unless we manually request it.
            window.request_redraw();
            let total_time = state.timer.total_time();
            if total_time - time_elapsed >= 1.0 {
                let fps = frame_count as f32 / (total_time - time_elapsed);
                window.set_title(format!("fps: {}", fps).as_str());

                // Reset for next average.
                frame_count = 0;
                time_elapsed = total_time;
            }
        },

        _ => {}
    });
}
