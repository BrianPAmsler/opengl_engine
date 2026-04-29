use gl_types::matrices::Mat4;
use gl_types::vectors::Vec3;
use embed_shader_source::embed_shader_source;
use rand::RngExt;

use crate::engine::graphics::builder::TextureBuilder;
use crate::engine::graphics::gl_enums::{InternalFormat, PixelFormat, PrimitiveType, TextureMagFilter, TextureMinFilter, TextureTarget, TextureUnit, TextureWrapMode};
use crate::engine::graphics::terrain::Terrain;
use crate::engine::graphics::{BufferedMesh, FragmentShader, GlUniformLocation, Graphics, Mesh, ShaderProgram, ShaderProgramBuilder, Texture, VBOBufferer, Vertex, VertexShader};
use crate::engine::errors::Result;


// Lays flat on the ground (+Z is "up")
const TERRAIN_CELL_VERTICES: &[Vertex] = &[
    // [0]: Bottom-Left Corner
    Vertex { x: 0.0, y: 0.0, z: 0.0 },
    // [1]: Bottom-Right Corner
    Vertex { x: 1.0, y: 0.0, z: 0.0 },
    // [2]: Top-Left Corner
    Vertex { x: 0.0, y: 0.0, z: 1.0 },
    // [3]: Top-Right Corner
    Vertex { x: 1.0, y: 0.0, z: 1.0 },
    // [4]: Center
    Vertex { x: 0.5, y: 0.0, z: 0.5 },
];

const TERRAIN_CELL_ELEMENTS: &[u32] = &[
    // -X side
    0,
    4,
    2,

    // +X side
    1,
    3,
    4,

    // -Z side
    0,
    1,
    4,

    // +Z side
    2,
    4,
    3,
];

pub struct TerrainRenderer {
    shader_program: ShaderProgram,
    mesh: BufferedMesh,
    vp_location: GlUniformLocation,
    terrain_dimensions_location: GlUniformLocation,
    height_scale_location: GlUniformLocation,
    view_pos_location: GlUniformLocation,
    pixel_size_location: GlUniformLocation,
    noise_map_size_location: GlUniformLocation,
    noise_texture: Texture
}

impl TerrainRenderer {
    pub fn new(gfx: &Graphics) -> Result<TerrainRenderer> {
        let mut shader_program = ShaderProgramBuilder::new(gfx);

        let vertex_shader_source = embed_shader_source!("terrain.vert");
        let fragment_shader_source = embed_shader_source!("terrain.frag");

        let vertex_shader = VertexShader::compile_shader(gfx, vertex_shader_source)?;
        let fragment_shader = FragmentShader::compile_shader(gfx, fragment_shader_source)?;

        shader_program.attach_shader(vertex_shader);
        shader_program.attach_shader(fragment_shader);

        let shader_program = shader_program.finish();

        gfx.glUseProgram(shader_program.program());

        let vp_location = gfx.glGetUniformLocation(shader_program.program(), "vp");
        let terrain_dimensions_location = gfx.glGetUniformLocation(shader_program.program(), "terrainDimensions");
        let height_scale_location = gfx.glGetUniformLocation(shader_program.program(), "heightScale");
        let view_pos_location = gfx.glGetUniformLocation(shader_program.program(), "viewPos");
        let pixel_size_location = gfx.glGetUniformLocation(shader_program.program(), "pixelSize");
        let noise_map_size_location = gfx.glGetUniformLocation(shader_program.program(), "noiseMapSize");

        let mesh = Mesh::new("Terrain Mesh".to_owned(), TERRAIN_CELL_VERTICES.to_owned().into_boxed_slice(), None, None, None, None);

        let mut vbo = VBOBufferer::new(gfx);
        let mesh = vbo.add_mesh(mesh);

        vbo.buffer_data(gfx);

        let mesh = mesh.take();

        let mut rng = rand::rng();
        let pixels: Vec<u8> = (0..1024u32.pow(2)).map(|_| rng.random()).collect();
        let noise_texture = unsafe { TextureBuilder::from_raw_pixels_unchecked(&pixels, 1024, 1024, InternalFormat::GL_RED, PixelFormat::GL_RED) }
            .mag_filter(TextureMagFilter::GL_NEAREST)
            .min_filter(TextureMinFilter::GL_NEAREST)
            .wrap_s(TextureWrapMode::GL_REPEAT)
            .wrap_t(TextureWrapMode::GL_REPEAT)
            .finish(gfx);

        Ok(TerrainRenderer { shader_program, mesh, vp_location, terrain_dimensions_location, height_scale_location, view_pos_location, pixel_size_location, noise_map_size_location, noise_texture })
    }

    pub fn render(&self, gfx: &Graphics, terrain: &mut Terrain, view_matrix: Mat4, projection_matrix: Mat4, camera_pos: Vec3) {
        gfx.glBindVertexArray(self.mesh.vao());
        gfx.glUseProgram(self.shader_program.program());
        terrain.update_textures(gfx);
        terrain.bind_textures(gfx);

        gfx.glActiveTexture(TextureUnit::GL_TEXTURE2);
        gfx.glBindTexture(TextureTarget::GL_TEXTURE_2D, self.noise_texture.texture_id());

        // uniform mat4 vp;
        let vp = projection_matrix * view_matrix;
        gfx.glUniformMatrix4f(self.vp_location, false, &vp);
        // uniform uvec2 terrainDimensions;
        gfx.glUniform2ui(self.terrain_dimensions_location, terrain.width(), terrain.height());
        // uniform float heightScale;
        gfx.glUniform1f(self.height_scale_location, 15.0);
        // uniform vec3 viewPos;
        gfx.glUniform3f(self.view_pos_location, camera_pos.x(), camera_pos.y(), camera_pos.z());
        gfx.glUniform1i(self.noise_map_size_location, self.noise_texture.width() as i32);

        gfx.glDrawElementsInstanced(PrimitiveType::GL_TRIANGLES, TERRAIN_CELL_ELEMENTS, terrain.width() * terrain.height());

        // todo!()
    }
}