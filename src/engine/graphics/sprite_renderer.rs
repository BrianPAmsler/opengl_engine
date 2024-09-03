use glm::Vec4;
use include_crypt_bytes::include_bytes_obfuscate;

use crate::engine::graphics::{Mesh, VBOBufferer, Vertex, UV};

use crate::engine::errors::Result;

use super::{graphics, BufferedMesh, FragmentShader, Graphics, ShaderProgram, ShaderProgramBuilder, VertexShader};

#[repr(align(16))]
struct Vec3(glm::Vec3);

#[repr(C, align(16))]
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

#[cfg(test)]
mod tests {
    use include_crypt_bytes::include_bytes_obfuscate;

    use crate::{engine::graphics::{sprite_renderer::{GLSpriteStruct, Vec3}, FragmentShader, Graphics, ShaderProgramBuilder, VertexShader}, vec3, vec4};

    use super::SpriteRenderer;


    #[test]
    pub fn alignment_test() {
        let mut test_struct = GLSpriteStruct {
            position: Vec3(vec3!(1, 2, 3)),
            dimensions: vec4!(4, 5, 6, 7),
            uvs: vec4!(8, 9, 10, 11),
            anchor: Vec3(vec3!(12, 13, 14)),
            id: unsafe { std::mem::transmute(15f32) },
        };

        let floats = unsafe { std::slice::from_raw_parts_mut::<f32>(&mut test_struct as *mut _ as *mut _, std::mem::size_of::<GLSpriteStruct>() / 4) };

        // zero any uninitialized memory
        floats[3] = 0.0f32;
        floats[15] = 0.0f32;
        floats[17] = 0.0f32;
        floats[18] = 0.0f32;
        floats[19] = 0.0f32;

        eprintln!("{:?}", floats);
        let expected = [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 0.0, 15.0, 0.0, 0.0, 0.0];
        
        assert_eq!(&floats, &expected);
    }

    #[allow(non_upper_case_globals)]
    #[test]
    pub fn sprite_struct_test() {
        let gfx = Graphics::init("test_window", 1289, 720, crate::engine::WindowMode::Windowed).unwrap();

        let renderer = SpriteRenderer::new(&gfx).unwrap();

        let mut program = ShaderProgramBuilder::new(&gfx);
        
        let vertex_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/sprite_struct_test.vert").unwrap()).unwrap();
        let fragment_shader_source = String::from_utf8(include_bytes_obfuscate!("src/engine/graphics/shaders/sprite.frag").unwrap()).unwrap();

        let vert_shader = VertexShader::compile_shader(&gfx, &vertex_shader_source).unwrap();
        let frag_shader = FragmentShader::compile_shader(&gfx, &fragment_shader_source).unwrap();

        program.attach_shader(vert_shader);
        program.attach_shader(frag_shader);

        let program = program.finish();

        gfx.glUseProgram(program.program());


        let mut buffer = 0;
        gfx.glGenBuffer(&mut buffer);

        let sprite_structs = [
            GLSpriteStruct {
                position: Vec3(vec3!(0)),
                dimensions: vec4!(0),
                uvs: vec4!(0),
                anchor: Vec3(vec3!(0)),
                id: 0,
            },
            GLSpriteStruct {
                position: Vec3(vec3!(0)),
                dimensions: vec4!(0),
                uvs: vec4!(0),
                anchor: Vec3(vec3!(0)),
                id: 0,
            }
        ];

        // gfx.glBindBuffer(GL_STORAGE, ssbo);
        // glBufferData(GL_SHADER_STORAGE_BUFFER, sizeof(data), data​, GLenum usage); //sizeof(data) only works for statically sized C/C++ arrays.
        // glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 3, ssbo);
        // glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0); // unbind


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

    pub fn render(gfx: &Graphics) {

    }
}