use glm::{Vec3, Vec4};
use include_crypt_bytes::include_bytes_obfuscate;

use crate::engine::graphics::{Mesh, VBOBufferer, Vertex, UV};

use crate::engine::errors::Result;

use super::{BufferedMesh, FragmentShader, Graphics, ShaderProgram, ShaderProgramBuilder, VertexShader};

#[repr(align(16))]
struct GLSpriteStruct {
    position: Vec3,
    dimensions: Vec4,
    uvs: Vec4,
    anchor: Vec3,
    id: u32
}

pub struct SpriteRenderer {
    program: ShaderProgram,
    mesh: BufferedMesh
}

mod tests {
    use super::GLSpriteStruct;

    #[test]
    pub fn alignment_test() {
        let test_struct = GLSpriteStruct {
            position: todo!(),
            dimensions: todo!(),
            uvs: todo!(),
            anchor: todo!(),
            id: todo!(),
        };
    }
}

// The include_bytes_obfuscate! macro generates non upper case globals and doesn't ignore the warning. wtf???
#[allow(non_upper_case_globals)]
impl SpriteRenderer {
    pub fn new(gfx: &Graphics) -> Result<SpriteRenderer> {
        let mut program = ShaderProgramBuilder::new(gfx);
        
        let vertex_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/sprite.vert").unwrap()).unwrap();
        let fragment_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/sprite.frag").unwrap()).unwrap();

        let vert_shader = VertexShader::compile_shader(gfx, &vertex_shader_source)?;
        let frag_shader = FragmentShader::compile_shader(gfx, &fragment_shader_source)?;

        program.attach_shader(vert_shader);
        program.attach_shader(frag_shader);

        let program = program.finish();

        let vertex_data = Box::new([
            Vertex { x: -1.0, y: -1.0, z: 0.0 }, // bottom left
            Vertex { x: -1.0, y: 1.0, z: 0.0 },  // top left
            Vertex { x: 1.0, y: -1.0, z: 0.0 },  // bottom right
        
            Vertex { x: -1.0, y: 1.0, z: 0.0 },  // top left
            Vertex { x: 1.0, y: 1.0, z: 0.0 },   // top right
            Vertex { x: 1.0, y: -1.0, z: 0.0 }   // bottom right
        ]);

        let uv_data = Box::new([
            UV { u: 0.0 , v: 1.0 },
            UV { u: 0.0 , v: 0.0 },
            UV { u: 1.0 , v: 1.0 },

            UV { u: 0.0 , v: 0.0 },
            UV { u: 1.0 , v: 0.0 },
            UV { u: 1.0 , v: 1.0 }
        ]);

        let mesh = Mesh::new("Sprite Mesh".to_owned(), vertex_data, None, Some(uv_data), None, None);
        
        let mut vbo = VBOBufferer::new(gfx);
        let mesh = vbo.add_mesh(mesh);

        vbo.buffer_data(gfx);

        let mesh = mesh.take();
        

        Ok(SpriteRenderer { program, mesh })
    }
}