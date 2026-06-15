use std::sync::Arc;

use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage}, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo}, image::{Image, sampler::Sampler, view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, sync::GpuFuture as _};

use crate::{engine::graphics::texture::error::BufferImageError, error::Result};

use super::Graphics;

#[derive(Clone)]
pub struct Texture {
    image: Arc<Image>,
    view: Arc<ImageView>,
    sampler: Arc<Sampler>,
    width: u32,
    height: u32
}

impl Texture {
    pub fn update_texture(&self, gfx: &Graphics, image_data: Vec<u8>) -> Result<(), BufferImageError> {
        buffer_to_image(gfx, image_data, &self.image)?;

        Ok(())
    }

    pub(in crate::engine::graphics) fn image(&self) -> &Arc<Image> {
        &self.image
    }

    pub(in crate::engine::graphics) fn view(&self) -> &Arc<ImageView> {
        &self.view
    }

    pub(in crate::engine::graphics) fn sampler(&self) -> &Arc<Sampler> {
        &self.sampler
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn buffer_to_image(gfx: &Graphics, image_data: Vec<u8>, image: &Arc<Image>) -> Result<(), BufferImageError> {
    let staging_buffer = Buffer::from_iter(
        gfx.memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        image_data,
    )?;
    
    let mut builder = AutoCommandBufferBuilder::primary(
        gfx.command_buffer_allocator.clone(),
        gfx.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )?;

    builder.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
        staging_buffer.clone(),
        image.clone(),
    ))?;

    let command_buffer = builder.build()?;
    let future = vulkano::sync::now(gfx.device.clone())
        .then_execute(gfx.queue.clone(), command_buffer)?
        .then_signal_fence_and_flush()?;

    future.wait(None)?;

    Ok(())
}

pub mod builder {
    use image::RgbaImage;
    use vulkano::{format::Format, image::{Image, ImageCreateInfo, ImageType, ImageUsage, sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode}, view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}};

    use crate::{engine::graphics::{Graphics, Texture, texture::{buffer_to_image, error::TextureBuilderError}}, error::Result};

    pub struct TextureBuilder {
        data: Vec<u8>,
        width: u32,
        height: u32,
        format: Format,
        wrap_s: SamplerAddressMode,
        wrap_t: SamplerAddressMode,
        min_filter: Filter,
        mag_filter: Filter,
    }

    impl TextureBuilder {
        pub fn from_image(image: RgbaImage) -> TextureBuilder {
            let (width, height) = image.dimensions();
            let data = image.into_raw();
            TextureBuilder {
                data,
                width,
                height,
                format: Format::R8G8B8A8_SRGB,
                wrap_s: SamplerAddressMode::Repeat,
                wrap_t: SamplerAddressMode::Repeat,
                min_filter: Filter::Linear,
                mag_filter: Filter::Linear,
            }
        }

        pub fn from_raw_pixels(data: Vec<u8>, width: u32, height: u32, format: Format) -> TextureBuilder {

            TextureBuilder {
                data,
                width,
                height,
                format,
                wrap_s: SamplerAddressMode::Repeat,
                wrap_t: SamplerAddressMode::Repeat,
                min_filter: Filter::Linear,
                mag_filter: Filter::Linear,
            }
        }

        pub fn wrap_s(mut self, wrap_s: SamplerAddressMode) -> Self {
            self.wrap_s = wrap_s;
            self
        }

        pub fn wrap_t(mut self, wrap_t: SamplerAddressMode) -> Self {
            self.wrap_t = wrap_t;
            self
        }

        pub fn min_filter(mut self, min_filter: Filter) -> Self {
            self.min_filter = min_filter;
            self
        }

        pub fn mag_filter(mut self, mag_filter: Filter) -> Self {
            self.mag_filter = mag_filter;
            self
        }

        pub fn finish(self, gfx: &Graphics) -> Result<Texture, TextureBuilderError> {
            let Self { data, width, height, format, wrap_s, wrap_t, min_filter, mag_filter } = self;

            let image = Image::new(
                gfx.memory_allocator.clone(),
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format,
                    extent: [width, height, 1],
                    usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                    ..Default::default()
                },
            )?;

            buffer_to_image(gfx, data, &image)?;

            let sampler = Sampler::new(
                gfx.device.clone(),
                SamplerCreateInfo {
                    mag_filter,
                    min_filter,
                    mipmap_mode: SamplerMipmapMode::Nearest,
                    address_mode: [wrap_s, wrap_t, SamplerAddressMode::Repeat],
                    mip_lod_bias: 0.0,
                    ..Default::default()
                },
            )?;

            let view = ImageView::new_default(image.clone())?;

            Ok(Texture { image, view, sampler, width, height  })
        }
    }
}

#[allow(clippy::enum_variant_names)]
pub mod error {
    use error_union::error_union;
    use vulkano::command_buffer::CommandBufferExecError;

    use crate::{engine::graphics::{sprite_renderer::error::AddSpritesheetError, terrain::{error::TerrainFromRawError, terrain_renderer::error::NewTerrainRendererError}}, error::EngineError};
    type ValidatedVulkanError = vulkano::Validated<vulkano::VulkanError>;
    type ValidatedAllocateBufferError = vulkano::Validated<vulkano::buffer::AllocateBufferError>;
    type BoxedValidationError = Box<vulkano::ValidationError>;
    type ValidatedAllocateImageError = vulkano::Validated<vulkano::image::AllocateImageError>;

    impl EngineError for Box<vulkano::ValidationError> {}
    impl EngineError for CommandBufferExecError {}
    impl EngineError for ValidatedAllocateImageError {}
    error_union!(ValidatedVulkanError, ValidatedAllocateBufferError, BoxedValidationError, CommandBufferExecError as BufferImageError into TextureBuilderError);
    error_union!(ValidatedAllocateImageError, ValidatedAllocateBufferError, BoxedValidationError, CommandBufferExecError, ValidatedVulkanError as TextureBuilderError into AddSpritesheetError, NewTerrainRendererError, TerrainFromRawError);
}