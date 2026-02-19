use gl_types::matrices::Mat4;
use gl_types::vectors::Vec3;
use gl46::{GL_TEXTURE_2D, GL_TEXTURE0, GL_TRIANGLES};

use crate::engine::graphics::terrain::Terrain;
use crate::engine::graphics::{BufferedMesh, FragmentShader, GlUniformLocation, Graphics, Mesh, ShaderProgram, ShaderProgramBuilder, VBOBufferer, Vertex, VertexShader, embed_shader_source};
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
    view_pos_location: GlUniformLocation
}

impl TerrainRenderer {
    pub fn new(gfx: &Graphics) -> Result<TerrainRenderer> {
        let mut shader_program = ShaderProgramBuilder::new(gfx);

        let vertex_shader_source = embed_shader_source!("src/engine/graphics/shaders/terrain.vert");
        let fragment_shader_source = embed_shader_source!("src/engine/graphics/shaders/terrain.frag");

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

        let mesh = Mesh::new("Terrain Mesh".to_owned(), TERRAIN_CELL_VERTICES.to_owned().into_boxed_slice(), None, None, None, None);

        let mut vbo = VBOBufferer::new(gfx);
        let mesh = vbo.add_mesh(mesh);

        vbo.buffer_data(gfx);

        let mesh = mesh.take();

        Ok(TerrainRenderer { shader_program, mesh, vp_location, terrain_dimensions_location, height_scale_location, view_pos_location })
    }

    pub fn render(&self, gfx: &Graphics, terrain: &mut Terrain, view_matrix: Mat4, projection_matrix: Mat4, camera_pos: Vec3) {
        terrain.update_textures(gfx);
        gfx.glBindVertexArray(self.mesh.vao());
        terrain.bind_textures(gfx);

        // uniform mat4 vp;
        let vp = projection_matrix * view_matrix;
        gfx.glUniformMatrix4f(self.vp_location, true, &vp);
        // uniform uvec2 terrainDimensions;
        gfx.glUniform2ui(self.terrain_dimensions_location, terrain.width(), terrain.height());
        // uniform float heightScale;
        gfx.glUniform1f(self.height_scale_location, 255.0);
        // uniform vec3 viewPos;
        gfx.glUniform3f(self.view_pos_location, camera_pos.x(), camera_pos.y(), camera_pos.z());

        gfx.glDrawElementsInstanced(GL_TRIANGLES, TERRAIN_CELL_ELEMENTS, terrain.width() * terrain.height());

        // todo!()
    }
}