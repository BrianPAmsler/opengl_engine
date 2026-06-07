use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use gl_types::{matrices::{Mat4, MatN}, vectors::{Vec3, VecN}};
use rand::RngExt as _;
use vulkano::{buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer, view}, command_buffer::DrawIndexedIndirectCommand, format::Format, image::sampler::{Filter, SamplerAddressMode}, memory::allocator::AllocationCreateInfo, padded::Padded, pipeline::graphics::vertex_input::Vertex, shader::ShaderModule};

use crate::engine::{errors::Result, graphics::{AlignedVec2, AlignedVec3, Binding, BufferType, Graphics, PipelineBuilder, PipelineHandle, Texture, builder::TextureBuilder, terrain::{INDEX_DATA, terrain_renderer::{fragment_shader::FragmentUniforms, vertex_shader::VertexUniforms}}}};

pub(in crate::engine::graphics::terrain) mod vertex_shader {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "src/engine/graphics/shaders/terrain.vert",
        root_path_env: "CARGO_MANIFEST_DIR"
    }
    
    #[allow(clippy::derivable_impls)]
    impl Default for VertexUniforms {
        fn default() -> Self {
            Self { vp: Default::default(), terrainDimensions: Default::default(), heightScale: Default::default() }
        }
    }
}

pub(in crate::engine::graphics::terrain) mod fragment_shader {
    use vulkano::padded::Padded;

    vulkano_shaders::shader!{
        ty: "fragment",
        path: "src/engine/graphics/shaders/terrain.frag",
        root_path_env: "CARGO_MANIFEST_DIR"
    }

    impl Default for FragmentUniforms {
        fn default() -> Self {
            Self { ambientIntensity: Padded(0.2), globalLightDir: Padded([-1.0, -1.0, -1.0]), viewPos: Default::default(), pixelSize: 0.05, noiseMapSize: 1 }
        }
    }
}

struct TerrainInfo {
    width: u32,
    height: u32,
    pipeline: PipelineHandle
}

pub struct TerrainRenderer {
    render_queue: Vec<TerrainInfo>,
    noise_texture: Texture,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
}

impl TerrainRenderer {
    pub fn new(gfx: &mut Graphics) -> Result<TerrainRenderer> {

        let vertex_shader = vertex_shader::load(gfx.device.clone())?;
        let fragment_shader = fragment_shader::load(gfx.device.clone())?;

        let mut rng = rand::rng();
        let pixels: Vec<u8> = (0..1024u32.pow(2)).map(|_| rng.random()).collect();
        let noise_texture = TextureBuilder::from_raw_pixels(pixels, 1024, 1024, Format::R8_UNORM)
            .mag_filter(Filter::Nearest)
            .min_filter(Filter::Nearest)
            .wrap_s(SamplerAddressMode::Repeat)
            .wrap_t(SamplerAddressMode::Repeat)
            .finish(gfx)?;

        Ok(TerrainRenderer { render_queue: Vec::new(), noise_texture, vertex_shader, fragment_shader })
    }

    pub fn queue_terrain(&mut self, width: u32, height: u32, pipeline: PipelineHandle) {
        self.render_queue.push(TerrainInfo { width, height, pipeline });
    }

    pub fn update(&mut self, gfx: &Graphics, view_matrix: Mat4, projection_matrix: Mat4, camera_pos: Vec3) {
        for terrain in self.render_queue.drain(..) {
            match gfx.get_binding(terrain.pipeline, 0) {
                Binding::Buffer(vertex_uniforms) => {
                    let vertex_uniforms = Subbuffer::new(vertex_uniforms).reinterpret::<VertexUniforms>();
                    *{#[allow(clippy::unwrap_used)] vertex_uniforms.write().unwrap()} = VertexUniforms {
                        vp: (projection_matrix * view_matrix).as_array(),
                        terrainDimensions: [terrain.width, terrain.height],
                        heightScale: 15.0,
                    };
                },
                _ => unreachable!()
            }

            match gfx.get_binding(terrain.pipeline, 1) {
                Binding::Buffer(fragment) => {
                    let fragment_uniforms = Subbuffer::new(fragment).reinterpret::<FragmentUniforms>();
                    
                    *{#[allow(clippy::unwrap_used)] fragment_uniforms.write().unwrap()} = FragmentUniforms {
                        viewPos: camera_pos.as_array(),
                        noiseMapSize: self.noise_texture.width() as i32,
                        ..Default::default()
                    };
                },
                _ => unreachable!()
            }

            let instance_count = terrain.width * terrain.height;
            gfx.set_indirect_buffer(terrain.pipeline, DrawIndexedIndirectCommand {
                index_count: INDEX_DATA.len() as u32,
                instance_count,
                first_index: 0,
                vertex_offset: 0,
                first_instance: 0,
            });
        }
    }

    pub(in crate::engine::graphics::terrain) fn vertex_shader(&self) -> &Arc<ShaderModule> {
        &self.vertex_shader
    }

    pub(in crate::engine::graphics::terrain) fn fragment_shader(&self) -> &Arc<ShaderModule> {
        &self.fragment_shader
    }

    pub(in crate::engine::graphics::terrain) fn noise_texture(&self) -> &Texture {
        &self.noise_texture
    }
}