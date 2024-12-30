use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::{scene::{GpuCamera, GpuMaterial, Scene}, utils::{StorageBuffer, UniformBuffer, Vertex}};


pub struct RenderContext<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    image_bind_group: wgpu::BindGroup,
    camera_buffer: UniformBuffer,
    scene_bind_group: wgpu::BindGroup,
    scene: Scene,
}

// const RGB_TRIANGLE: &[Vertex] = &[
//     Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
//     Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
// ];

// TODO: Pass from 2 triangles to 1 triangle
// https://webgpufundamentals.org/webgpu/lessons/webgpu-large-triangle-to-cover-clip-space.html

// https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#the-results
// tex needs to be flipped somehow
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0,1.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
];

const VERTICES_LEN: usize = VERTICES.len();

impl<'a> RenderContext<'a> {
    pub async fn new(
        window: &'a Window,
        scene: &Scene,
    ) -> RenderContext<'a> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_defaults()
                } else {
                    wgpu::Limits {
                        max_storage_buffer_binding_size: 512_u32 << 20,
                        ..Default::default()
                    }
                },
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let camera_buffer = {
            let camera = GpuCamera::new(
                &scene.camera,
                (
                    size.width, 
                    size.height
                )
            );

            UniformBuffer::new_from_bytes(
                &device,
                bytemuck::bytes_of(&camera),
                0_u32,
                Some("camera buffer"),
            )
        };

        let (scene_bind_group_layout, scene_bind_group) = {
            let sphere_buffer = StorageBuffer::new_from_bytes(
                &device,
                bytemuck::cast_slice(scene.spheres.as_slice()),
                0_u32,
                Some("scene buffer"),
            );
            
            let mut global_texture_data = Vec::new();
            let mut material_data: Vec<GpuMaterial> = Vec::with_capacity(scene.materials.len());
            for material in scene.materials.iter() {
                material_data.push(GpuMaterial::new(material, &mut global_texture_data));
            }

            let material_buffer = StorageBuffer::new_from_bytes(
                &device,
                bytemuck::cast_slice(material_data.as_slice()),
                1_u32,
                Some("material buffer"),
            );

            let texture_buffer = StorageBuffer::new_from_bytes(
                &device,
                bytemuck::cast_slice(global_texture_data.as_slice()),
                2_u32,
                Some("texture buffer"),
            );

            let scene_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    sphere_buffer.layout(wgpu::ShaderStages::FRAGMENT, true),
                    material_buffer.layout(wgpu::ShaderStages::FRAGMENT, true),
                    texture_buffer.layout(wgpu::ShaderStages::FRAGMENT, true),
                ],
                label: Some("scene layout"),
            });
            
            let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &scene_bind_group_layout,
                entries: &[
                    sphere_buffer.binding(),
                    material_buffer.binding(),
                    texture_buffer.binding(),
                ],
                label: Some("scene bind group"),
            });

            (scene_bind_group_layout, scene_bind_group)
        };

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader/raytracing.wgsl"),
        );

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let image_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                camera_buffer.layout(wgpu::ShaderStages::FRAGMENT),
            ],
            label: Some("image layout"),
        });

        let image_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &image_bind_group_layout,
            entries: &[
                camera_buffer.binding(),
            ],
            label: Some("image bind group"),
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &image_bind_group_layout,
                &scene_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        Vertex::desc(),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            }
        );

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            image_bind_group,
            camera_buffer,
            scene_bind_group,
            scene: scene.clone(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ => false,
        }
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.012,
                                g: 0.012,
                                b: 0.012,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            {
                let camera = GpuCamera::new(
                    &self.scene.camera,
                    (
                        self.size.width,
                        self.size.height
                    )
                );
                self.queue.write_buffer(
                    &self.camera_buffer.handle(),
                    0,
                    bytemuck::bytes_of(&camera),
                );
            }

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.image_bind_group, &[]);
            render_pass.set_bind_group(1, &self.scene_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..VERTICES_LEN as u32, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())

    }
}

 

 