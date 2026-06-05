use std::{any::TypeId, collections::HashMap, iter, marker::PhantomData, sync::Arc};

use bytemuck::Pod;
use itertools::Itertools;
use vulkano::{Validated, VulkanError, VulkanLibrary, buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer}, command_buffer::{AutoCommandBufferBuilder, CommandBuffer, CommandBufferUsage, DrawIndexedIndirectCommand, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo, allocator::StandardCommandBufferAllocator}, descriptor_set::{self, DescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator}, device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags, physical::PhysicalDeviceType}, format::{ClearValue, Format}, image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}, pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{ColorBlendAttachmentState, ColorBlendState}, depth_stencil::{DepthState, DepthStencilState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::{CullMode, FrontFace, PolygonMode, RasterizationState}, vertex_input::{Vertex, VertexBufferDescription, VertexDefinition as _, VertexInputState}, viewport::{Viewport, ViewportState}}, layout::PipelineDescriptorSetLayoutCreateInfo}, render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass}, shader::ShaderModule, swapchain::{self, PresentMode, Surface, Swapchain, SwapchainCreateFlags, SwapchainCreateInfo, SwapchainPresentInfo}, sync::{self, GpuFuture}};
use winit::{event_loop::EventLoop, window::Window};
use crate::engine::{WindowMode, data_structures::{AllocationIndex, VecAllocator}, errors::Result, graphics::Texture};

unsafe fn exit<T> (status: i32) -> T {
    std::process::exit(status)
}

pub trait UnsizedBuffer: BufferContents {
    fn size(count: u64) -> u64;
}

#[derive(Clone, Copy)]
pub struct PipelineHandle {
    handle: AllocationIndex 
}

struct PipelineCell {
    pipeline: Arc<GraphicsPipeline>,
    bindings: HashMap<u32, Binding>,
    vertex_buffer_description: VertexBufferDescription,
    vertex_buffer: Arc<Buffer>,
    index_buffer: Arc<Buffer>,
    indirect_buffer: Subbuffer<[DrawIndexedIndirectCommand]>,
    descriptor_set: Arc<DescriptorSet>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    vertex_shader: Arc<ShaderModule>,
    intermediate_shaders: Vec<Arc<ShaderModule>>,
    fragment_shader: Arc<ShaderModule>
}

pub mod pipeline_builder {
    pub mod stage_1 {
        use std::sync::Arc;

        use vulkano::shader::ShaderModule;

        use crate::engine::graphics::Graphics;

        pub struct PipelineBuilder<'a> {
            pub(in crate::engine::graphics::graphics) gfx: &'a mut Graphics,
        }

        impl PipelineBuilder<'_> {
            pub fn new(gfx: &mut Graphics) -> PipelineBuilder<'_> {
                PipelineBuilder { gfx }
            }
        }

        impl<'a> PipelineBuilder<'a> {
            pub fn vertex_shader(self, shader_module: Arc<ShaderModule>) -> super::stage_2::PipelineBuilder<'a> {
                let Self { gfx } = self;
                let vertex_shader = shader_module;
                super::stage_2::PipelineBuilder {
                    gfx,
                    vertex_shader,
                    shaders: Vec::new()
                }
            }
        }
    }

    mod stage_2 {
        use std::sync::Arc;

        use vulkano::shader::ShaderModule;

        use crate::engine::graphics::Graphics;

        pub struct PipelineBuilder<'a> {
            pub(in crate::engine::graphics::graphics) gfx: &'a mut Graphics,
            pub(in crate::engine::graphics::graphics) vertex_shader: Arc<ShaderModule>,
            pub(in crate::engine::graphics::graphics) shaders: Vec<Arc<ShaderModule>>
        }

        impl<'a> PipelineBuilder<'a> {
            pub fn add_shader(mut self, shader_module: Arc<ShaderModule>) -> super::stage_2::PipelineBuilder<'a> {
                self.shaders.push(shader_module);

                self
            }

            pub fn fragment_shader(self, shader_module: Arc<ShaderModule>) -> super::stage_3::PipelineBuilder<'a> {
                let Self { gfx, vertex_shader, shaders } = self;
                let fragment_shader = shader_module;
                super::stage_3::PipelineBuilder {
                    gfx,
                    vertex_shader,
                    intermediate_shaders: shaders,
                    fragment_shader,
                }
            }
        }
    }

    mod stage_3 {
        use std::{collections::HashMap, sync::Arc};

        use vulkano::{buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::{PipelineShaderStageCreateInfo, graphics::vertex_input::{Vertex, VertexDefinition as _}}, shader::ShaderModule};

        use crate::engine::{errors::Result, graphics::{Graphics, graphics::get_pipeline}};

        pub struct PipelineBuilder<'a> {
            pub(in crate::engine::graphics::graphics) gfx: &'a mut Graphics,
            pub(in crate::engine::graphics::graphics) vertex_shader: Arc<ShaderModule>,
            pub(in crate::engine::graphics::graphics) intermediate_shaders: Vec<Arc<ShaderModule>>,
            pub(in crate::engine::graphics::graphics) fragment_shader: Arc<ShaderModule>,
        }

        impl<'a> PipelineBuilder<'a> {
            pub fn vertex_data<T: Vertex + BufferContents>(self, vertices: Vec<T>, indices: Vec<u32>) -> Result<super::stage_4::PipelineBuilder<'a>> {
                let Self { gfx, vertex_shader, intermediate_shaders, fragment_shader } = self;

                let vertex_buffer = Buffer::from_iter(
                    gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::VERTEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    vertices,
                )?
                .buffer()
                .clone();

                let index_buffer = Buffer::from_iter(
                    gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::INDEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    indices,
                )?
                .buffer()
                .clone();

                let vertex_buffer_description = T::per_vertex();

                Ok(super::stage_4::PipelineBuilder {
                    gfx,
                    vertex_buffer,
                    index_buffer,
                    vertex_shader,
                    intermediate_shaders,
                    fragment_shader,
                    vertex_buffer_description,
                    bindings: HashMap::new(),
                })
            }
        }
    }

    mod stage_4 {
        use std::{collections::HashMap, sync::Arc};

        use vulkano::{buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer}, command_buffer::{DrawIndexedIndirectCommand, DrawIndirectCommand}, descriptor_set, format::{self, Format}, image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::{GraphicsPipeline, PipelineShaderStageCreateInfo, graphics::vertex_input::{VertexBufferDescription, VertexInputState}}, shader::{ShaderModule, spirv::ImageFormat}};

        use crate::engine::{errors::Result, graphics::{Binding, BufferType, Graphics, PipelineHandle, Texture, UnsizedBuffer, graphics::{PipelineCell, get_command_buffers, get_descriptor_set, get_pipeline}}};

        pub struct PipelineBuilder<'a> {
            pub(in crate::engine::graphics) gfx: &'a mut Graphics,
            pub(in crate::engine::graphics) vertex_buffer: Arc<Buffer>,
            pub(in crate::engine::graphics) index_buffer: Arc<Buffer>,
            pub(in crate::engine::graphics) vertex_shader: Arc<ShaderModule>,
            pub(in crate::engine::graphics) intermediate_shaders: Vec<Arc<ShaderModule>>,
            pub(in crate::engine::graphics) fragment_shader: Arc<ShaderModule>,
            pub(in crate::engine::graphics) vertex_buffer_description: VertexBufferDescription,
            pub(in crate::engine::graphics) bindings: HashMap<u32, Binding>,
        }

        impl<'a> PipelineBuilder<'a> {
            pub fn add_uniform_buffer<T: BufferContents>(mut self, binding: u32, buffer: T, buffer_type: BufferType) -> Result<Self> {
                let buffer = Buffer::from_data(
                    self.gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: buffer_type.into(),
                        ..Default::default()
                    },
                    buffer,
                )?
                .buffer()
                .clone();

                self.bindings.insert(binding, Binding::Buffer(buffer));

                Ok(self)
            }

            pub fn add_storage_buffer<T: BufferContents>(mut self, binding: u32, buffer: T, buffer_type: BufferType) -> Result<Self> {
                let buffer = Buffer::from_data(
                    self.gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::STORAGE_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: buffer_type.into(),
                        ..Default::default()
                    },
                    buffer,
                )?
                .buffer()
                .clone();

                self.bindings.insert(binding, Binding::Buffer(buffer));

                Ok(self)
            }

            pub fn add_storage_buffer_unsized<T: BufferContents + ?Sized>(mut self, binding: u32, count: u64, buffer_type: BufferType) -> Result<Self> {
                let buffer = Buffer::new_unsized::<T>(
                    self.gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::STORAGE_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: buffer_type.into(),
                        ..Default::default()
                    },
                    count,
                )?
                .buffer()
                .clone();

                self.bindings.insert(binding, Binding::Buffer(buffer));

                Ok(self)
            }

            pub fn add_texture(mut self, binding: u32, texture: Texture) -> Result<Self> {
                self.bindings.insert(binding, Binding::Texture(texture));

                Ok(self)
            }

            pub fn finish(self) -> Result<PipelineHandle> {
                let Self { gfx, vertex_buffer, index_buffer, vertex_shader, intermediate_shaders, fragment_shader, vertex_buffer_description, bindings } = self;

                let pipeline = get_pipeline(
                    &gfx.device,
                    vertex_buffer_description.clone(),
                    &vertex_shader,
                    &intermediate_shaders,
                    &fragment_shader,
                    &gfx.render_pass,
                    gfx.viewport.clone()
                );

                let descriptor_set = get_descriptor_set(
                    &gfx.descriptor_set_allocator,
                    &pipeline,
                    &bindings
                )?;

                let indirect_buffer = Buffer::from_iter(
                    gfx.memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::INDIRECT_BUFFER | BufferUsage::TRANSFER_DST,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    [DrawIndexedIndirectCommand {
                        index_count: 0,
                        instance_count: 0,
                        first_index: 0,
                        vertex_offset: 0,
                        first_instance: 0,
                    }]
                )?;

                let command_buffers = get_command_buffers(
                    &gfx.command_buffer_allocator,
                    &gfx.queue,
                    &pipeline,
                    &descriptor_set,
                    &gfx.framebuffers,
                    &vertex_buffer,
                    &index_buffer,
                    &indirect_buffer
                );

                let cell = PipelineCell {
                    pipeline,
                    bindings,
                    vertex_buffer_description,
                    vertex_buffer,
                    index_buffer,
                    indirect_buffer,
                    descriptor_set,
                    command_buffers,
                    vertex_shader,
                    intermediate_shaders,
                    fragment_shader,
                };

                let handle = gfx.pipelines.insert(cell);
                Ok(PipelineHandle { handle })
            }
        }
    }
}

pub use pipeline_builder::stage_1::PipelineBuilder;

#[derive(Clone)]
pub enum Binding {
    Buffer(Arc<Buffer>),
    Texture(Texture)
}

pub enum BufferType {
    Static,
    Dynamic,
    DynamicRandomAccess
}

impl Into<MemoryTypeFilter> for BufferType {
    fn into(self) -> MemoryTypeFilter {
        match self {
            BufferType::Static => MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            BufferType::Dynamic => MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            BufferType::DynamicRandomAccess => MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
        }
    }
}

pub struct Graphics {
    todo: (), // TODO: make getter methods instead of exposing these to the whole crate
    pub(in crate::engine::graphics) vulkan_instance: Arc<Instance>,
    pub(in crate::engine::graphics) device: Arc<Device>,
    pub(in crate::engine::graphics) queue: Arc<Queue>,
    pub(in crate::engine::graphics) swapchain: Arc<Swapchain>,
    pub(in crate::engine::graphics) images: Vec<Arc<Image>>,
    pub(in crate::engine::graphics) render_pass: Arc<RenderPass>,
    pub(in crate::engine::graphics) framebuffers: Vec<Arc<Framebuffer>>,
    pub(in crate::engine::graphics) pipelines: VecAllocator<PipelineCell>,
    pub(in crate::engine::graphics) memory_allocator: Arc<StandardMemoryAllocator>,
    pub(in crate::engine::graphics) command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub(in crate::engine::graphics) descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub(in crate::engine::graphics) viewport: Viewport,
    pub(in crate::engine::graphics) window_resized: bool,
    pub(in crate::engine::graphics) recreate_swapchain: bool
}

fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                // Set the format the same as the swapchain.
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
            depth: {
                format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth},
        },
    )
    .unwrap()
}

fn get_framebuffers(memory_allocator: &Arc<StandardMemoryAllocator>, render_pass: Arc<RenderPass>, images: &[Arc<Image>]) -> Vec<Arc<Framebuffer>> {
    let [width, height, _] = images[0].extent();
    let depth_buffer = ImageView::new_default(
        Image::new(memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: [width, height, 1],
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        ).unwrap()
    )
    .unwrap();
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_buffer.clone()],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

fn get_pipeline(device: &Arc<Device>, vertex_buffer_description: VertexBufferDescription, vertex_shader: &Arc<ShaderModule>, intermediate_shaders: &Vec<Arc<ShaderModule>>, fragment_shader: &Arc<ShaderModule>, render_pass: &Arc<RenderPass>, viewport: Viewport) -> Arc<GraphicsPipeline> {
    let vertex_shader = vertex_shader.entry_point("main").unwrap();
    let fragment_shader = fragment_shader.entry_point("main").unwrap();
    
    let vertex_input_state = vertex_buffer_description
        .definition(&vertex_shader)
        .unwrap();

    let stages: Vec<_> = iter::once(vertex_shader)
        .chain(
            intermediate_shaders.iter()
                .map(|shader| shader.entry_point("main").unwrap())
        )
        .chain(iter::once(fragment_shader))
        .map(|entry_point| PipelineShaderStageCreateInfo::new(entry_point))
        .collect();

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

    GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState {
                viewports: [viewport].into_iter().collect(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState {
                cull_mode: CullMode::None,
                front_face: FrontFace::Clockwise,
                ..Default::default()
            }),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                subpass.num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),
            subpass: Some(subpass.into()),
            depth_stencil_state: Some(DepthStencilState {
                depth: Some(DepthState::reverse()),
                ..Default::default()
            }),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .unwrap()
}

fn get_descriptor_set(descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>, pipeline: &Arc<GraphicsPipeline>, bindings: &HashMap<u32, Binding>) -> Result<Arc<DescriptorSet>> {
    let descriptor_set_allocator = descriptor_set_allocator.clone();
    let pipeline_layout = pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();
    let descriptor_writes = bindings.iter()
        .map(|(binding, buffer)| {
            match buffer {
                Binding::Buffer(buffer) => WriteDescriptorSet::buffer(*binding, Subbuffer::new(buffer.clone())),
                Binding::Texture(texture) => WriteDescriptorSet::image_view_sampler(*binding, texture.view().clone(), texture.sampler().clone()),
            }
        });
    
    Ok(DescriptorSet::new(
        descriptor_set_allocator,
        descriptor_set_layout.clone(),
        descriptor_writes,
        [],
    )?)
}

fn get_command_buffers(command_buffer_allocator: &Arc<StandardCommandBufferAllocator>, queue: &Arc<Queue>, pipeline: &Arc<GraphicsPipeline>, descriptor_set: &Arc<DescriptorSet>, framebuffers: &Vec<Arc<Framebuffer>>, vertex_buffer: &Arc<Buffer>, index_buffer: &Arc<Buffer>, indirect_buffer: &Subbuffer<[DrawIndexedIndirectCommand]>, ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .map(|framebuffer| {
            let command_buffer_allocator = command_buffer_allocator.clone();
            let mut builder = AutoCommandBufferBuilder::primary(
                command_buffer_allocator,
                queue.queue_family_index(),
                // Don't forget to write the correct buffer usage.
                CommandBufferUsage::MultipleSubmit,
            )
            .unwrap();

            unsafe { builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.75, 0.75, 0.75, 1.0].into()), Some(ClearValue::Depth(0.0))],
                        ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                )
                .unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_vertex_buffers(0, Subbuffer::new(vertex_buffer.clone()))
                .unwrap()
                .bind_index_buffer(Subbuffer::new(index_buffer.clone()).cast_aligned::<u32>())
                .unwrap()
                .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.layout().clone(), 0, vec![descriptor_set.clone()])
                .unwrap()
                .draw_indexed_indirect(indirect_buffer.clone())
                .unwrap()
                .end_render_pass(SubpassEndInfo::default())
                .unwrap();
            }

            builder.build().unwrap()
        })
        .collect()
}

impl Graphics {
    pub fn new(window: Arc<Window>, event_loop: &EventLoop<()>) -> Result<Graphics> {
        let library = VulkanLibrary::new()?;
        let required_extensions = Surface::required_extensions(&event_loop)?;
        let vulkan_instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )?;

        let dimensions = window.inner_size();
        let surface = Surface::from_window(vulkan_instance.clone(), window)?;

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        
        let (physical_device, queue_family_index) = vulkan_instance
            .enumerate_physical_devices()?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    // Find the first first queue family that is suitable.
                    // If none is found, `None` is returned to `filter_map`,
                    // which disqualifies this physical device.
                    .position(|(i, q)| {
                        q.queue_flags.contains(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            }).min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,

                // Note that there exists `PhysicalDeviceType::Other`, however,
                // `PhysicalDeviceType` is a non-exhaustive enum. Thus, one should
                // match wildcard `_` to catch all unknown device types.
                _ => 4,
            })
            .ok_or("No physical devices.")?;

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        )?;

        let queue = queues.next().ok_or("No device queue.")?;

        let caps = physical_device
            .surface_capabilities(&surface, Default::default())?;
        
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format =  physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                present_mode: PresentMode::Immediate,
                min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
                image_format,
                image_extent: dimensions.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT, // What the images are going to be used for
                composite_alpha,
                ..Default::default()
            },
        )?;

        let render_pass = get_render_pass(device.clone(), swapchain.clone());

        // Flip viewport so +y is up (vulkan defaults to +y down)
        let viewport = Viewport {
            offset: [0.0, dimensions.height as f32],
            extent: [dimensions.width as f32, -(dimensions.height as f32)],
            depth_range: 0.0..=1.0,
        };

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let frame_buffers = get_framebuffers(&&memory_allocator, render_pass.clone(), &images);
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(device.clone(), Default::default()));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), Default::default()));

        // unsafe { gl.glPixelStorei(PixelStoreParameter::GL_UNPACK_ALIGNMENT, 1) };

        Ok(Graphics {
            todo: (),
            vulkan_instance,
            device,
            queue,
            swapchain,
            images,
            render_pass,
            framebuffers: frame_buffers,
            pipelines: VecAllocator::new(),
            memory_allocator,
            command_buffer_allocator,
            descriptor_set_allocator,
            viewport,
            window_resized: false,
            recreate_swapchain: false,
        })
    }

    pub fn validate_pipelines(&mut self, window: &Window) -> Result<()> {
        if self.window_resized || self.recreate_swapchain {
            self.recreate_swapchain = false;

            let new_dimensions = window.inner_size();

            let (new_swapchain, new_images) = self.swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent: new_dimensions.into(),
                    ..self.swapchain.create_info()
                })?;
            self.swapchain = new_swapchain;
            let new_framebuffers = get_framebuffers(&self.memory_allocator, self.render_pass.clone(), &new_images);

            if self.window_resized {
                self.window_resized = false;

                for (_, pipeline) in &mut self.pipelines {
                    let new_pipeline = get_pipeline(
                        &self.device,
                        pipeline.vertex_buffer_description.clone(),
                        &pipeline.vertex_shader,
                        &pipeline.intermediate_shaders,
                        &pipeline.fragment_shader,
                        &self.render_pass,
                        self.viewport.clone(),
                    );

                    let new_command_buffers = get_command_buffers(
                        &self.command_buffer_allocator,
                        &self.queue,
                        &new_pipeline,
                        &pipeline.descriptor_set,
                        &new_framebuffers,
                        &pipeline.vertex_buffer,
                        &pipeline.index_buffer,
                        &pipeline.indirect_buffer
                    );

                    pipeline.pipeline = new_pipeline;
                    pipeline.command_buffers = new_command_buffers;
                }
            }
        }

        Ok(())
    }

    pub fn remove_pipeline(&mut self, pipeline: PipelineHandle) -> Result<()> {
        self.pipelines.remove(pipeline.handle).map_err(|_| "Pipeline doesn't exist.")?;

        Ok(())
    }

    pub fn draw(&mut self) {
        let (image_i, suboptimal, acquire_future) =
        match swapchain::acquire_next_image(self.swapchain.clone(), None)
            .map_err(Validated::unwrap)
        {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        // This is cursed as fuck, but idk what else to do.
        let execution: Box<dyn GpuFuture> = Box::new(
            sync::now(self.device.clone())
                .join(acquire_future)
        );

        let pipelines = self.pipelines.iter();

        let execution = pipelines.fold(execution, |execution, (_, pipeline)| {
            Box::new(execution.then_execute(self.queue.clone(), pipeline.command_buffers[image_i as usize].clone()).unwrap())
        });
        
        let execution = execution
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            Ok(future) => {
                // Wait for the GPU to finish.
                future.wait(None).unwrap();
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
            }
            Err(e) => {
                println!("failed to flush future: {e}");
            }
        }
    }

    pub fn set_indirect_buffer(&self, pipeline: PipelineHandle, draw_command: DrawIndexedIndirectCommand) {
        let todo: (); // TODO: Remove unwraps
        let pipeline = self.pipelines.get(pipeline.handle).unwrap();
        let mut buffer = pipeline.indirect_buffer.write().unwrap();

        buffer[0] = draw_command;
    }

    // pub fn set_uniforms<T: BufferContents>(&self, pipeline: &PipelineHandle, uniforms: T) {
    //     let todo: (); // TODO: Remove unwraps
    //     let pipeline = self.pipelines.get(pipeline.handle).unwrap();
    //     let mut buffer = pipeline.bindings.write().unwrap();

    //     buffer[0] = draw_command;
    // }

    pub fn get_binding(&self, pipeline: PipelineHandle, binding: u32) -> Binding {
        let todo: (); // TODO: Remove unwraps
        let pipeline = self.pipelines.get(pipeline.handle).unwrap();

        pipeline.bindings.get(&binding).unwrap().clone()
    }

    pub fn window_resized(&mut self) {
        self.window_resized = true;
    }

    pub fn swap_buffers(&mut self) {
        // self.window.swap_buffers();
        todo!()
    }

    // pub fn flush_messages(&self) -> std::vec::IntoIter<(f64, WindowEvent)> {
    //     glfw::flush_messages(&self.events).collect::<Vec<(f64, WindowEvent)>>().into_iter()
    // }

    // pub fn set_fullscreen(&mut self, monitor: Monitor) {
    //     todo!()
    // }

    // pub fn is_supported(&mut self, gl_fn_name: &'static str) -> bool {
    //     self.window.get_proc_address(&gl_fn_name).is_null().not()
    // }

    // // This will be deleted once window is properly wrapped
    // pub fn __get_window(&self) -> &PWindow {
    //     &self.window
    // }

    // // This will be deleted once window is properly wrapped
    // pub fn __get_window_mut(&mut self) -> &mut PWindow {
    //     &mut self.window
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::engine::graphics::{Graphics, gl_enums::TextureUnit};

//     #[test]
//     #[ignore="requires user interaction"]
//     fn gl_unsupported() {
//         let lock = super::super::test_lock::LOCK.lock().unwrap();
//         let gfx = Graphics::init_unsupported().unwrap();

//         gfx.glActiveTexture(TextureUnit::GL_TEXTURE0);
//         drop(gfx);
//         drop(lock);
//     }
// }
