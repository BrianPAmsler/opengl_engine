use gl46::GL_TRIANGLES;
use glm::{Vec3, Vec4};

use crate::engine::graphics::{Mesh, VBOBufferer, Vertex, UV};

use crate::engine::errors::Result;

use super::{embed_shader_source, BufferedMesh, FragmentShader, Graphics, ShaderProgram, ShaderProgramBuilder, VertexShader};

#[repr(align(16))]
#[derive(Clone, Copy, PartialEq)]
struct AlignedVec3(glm::Vec3);

impl std::fmt::Debug for AlignedVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
struct GLSpriteStruct {
    position: AlignedVec3,
    dimensions: Vec4,
    uvs: Vec4,
    anchor: Vec3,
    id: u32
}

pub struct SpriteRenderer {
    program: ShaderProgram,
    mesh: BufferedMesh
}

impl SpriteRenderer {
    pub fn new(gfx: &Graphics) -> Result<SpriteRenderer> {
        let mut program = ShaderProgramBuilder::new(gfx);
        
        let vertex_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite.vert");
        let fragment_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite.frag");

        let vert_shader = VertexShader::compile_shader(gfx, vertex_shader_source)?;
        let frag_shader = FragmentShader::compile_shader(gfx, fragment_shader_source)?;

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

    fn buffer_sprite_data(&self, gfx: &Graphics) {
        
    } 

    pub fn render(&self, gfx: &Graphics) {
        gfx.glBindVertexArray(self.mesh.vao());
        gfx.glDrawArrays(GL_TRIANGLES, 0, self.mesh.len() as _);
    }
}

#[cfg(test)]
mod tests {
    use gl46::{GL_DYNAMIC_DRAW, GL_RGBA, GL_SHADER_STORAGE_BUFFER, GL_UNSIGNED_INT};
    
    impl PartialEq for GLSpriteStruct {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position && self.dimensions == other.dimensions && self.uvs == other.uvs && self.anchor == other.anchor && self.id == other.id
        }
    }

    use crate::{engine::graphics::{embed_shader_source, sprite_renderer::{GLSpriteStruct, AlignedVec3}, FragmentShader, Graphics, ShaderProgramBuilder, VertexShader}, vec3, vec4};

    use super::SpriteRenderer;

    #[test]
    pub fn alignment_test() {
        let mut sprite_structs = [
            GLSpriteStruct {
                position: AlignedVec3(vec3!(1, 2, 3)),
                dimensions: vec4!(4, 5, 6, 7),
                uvs: vec4!(8, 9, 10, 11),
                anchor: vec3!(12, 13, 14),
                id: unsafe { std::mem::transmute(15f32) }
            },
            GLSpriteStruct {
                position: AlignedVec3(vec3!(16, 17, 18)),
                dimensions: vec4!(19, 20, 21, 22),
                uvs: vec4!(23, 24, 25, 26),
                anchor: vec3!(27, 28, 29),
                id: unsafe { std::mem::transmute(30f32) }
            },
        ];

        let floats = unsafe { std::slice::from_raw_parts_mut::<f32>(&mut sprite_structs as *mut _ as *mut _, sprite_structs.len() * std::mem::size_of::<GLSpriteStruct>() / 4) };

        // zero any uninitialized memory
        floats[3] = 0.0f32;
        floats[19] = 0.0f32;

        eprintln!("{:?}", floats);
        let expected =
            [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0, 18.0, 0.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
        
        assert_eq!(&floats, &expected);
    }

    #[test]
    pub fn sprite_struct_test() {
        let gfx = Graphics::init("test_window", 1289, 720, crate::engine::WindowMode::Windowed).unwrap();

        let renderer = SpriteRenderer::new(&gfx).unwrap();

        let mut program = ShaderProgramBuilder::new(&gfx);
        
        let vertex_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite_struct_test.vert");
        let fragment_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite_struct_test.frag");

        let vert_shader = VertexShader::compile_shader(&gfx, vertex_shader_source).unwrap();
        let frag_shader = FragmentShader::compile_shader(&gfx, fragment_shader_source).unwrap();

        program.attach_shader(vert_shader);
        program.attach_shader(frag_shader);

        let program = program.finish();

        gfx.glUseProgram(program.program());

        let mut ssbo = 0;
        gfx.glGenBuffer(&mut ssbo);

        let sprite_structs = [GLSpriteStruct {
            position: AlignedVec3(vec3!(0)),
            dimensions: vec4!(0),
            uvs: vec4!(0),
            anchor: vec3!(0),
            id: 0,
        }; 2];

        let mut data_in = [GLSpriteStruct {
            position: AlignedVec3(vec3!(0)),
            dimensions: vec4!(0),
            uvs: vec4!(0),
            anchor: vec3!(0),
            id: 0,
        }; 2];

        const SPRITE_COUNT_ALIGNMENT: isize = 16;

        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo);
        // Allocate space
        gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, std::mem::size_of::<GLSpriteStruct>() * sprite_structs.len() + SPRITE_COUNT_ALIGNMENT as usize, GL_DYNAMIC_DRAW); 
        // Buffer length data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, 0, std::slice::from_ref(&sprite_structs.len())); 
        // Buffer sprite data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, SPRITE_COUNT_ALIGNMENT,&sprite_structs); 
        gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, ssbo);

        gfx.glUseProgram(program.program());
        renderer.render(&gfx);

        unsafe { gfx.glGetBufferSubData(GL_SHADER_STORAGE_BUFFER, SPRITE_COUNT_ALIGNMENT, std::mem::size_of::<GLSpriteStruct>() as isize * sprite_structs.len() as isize, data_in.as_mut_ptr() as *mut _) };

        println!("{:?}", data_in); 

        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0); // unbind
        
        let expected = [
            GLSpriteStruct {
                position: AlignedVec3(vec3!(1, 2, 3)),
                dimensions: vec4!(4, 5, 6, 7),
                uvs: vec4!(8, 9, 10, 11),
                anchor: vec3!(12, 13, 14),
                id: 15
            },
            GLSpriteStruct {
                position: AlignedVec3(vec3!(16, 17, 18)),
                dimensions: vec4!(19, 20, 21, 22),
                uvs: vec4!(23, 24, 25, 26),
                anchor: vec3!(27, 28, 29),
                id: 30
            }
        ];

        assert_eq!(data_in, expected);
    }

    #[test]
    fn test_unsupported() {
        let gfx = Graphics::init("test_window", 1289, 720, crate::engine::WindowMode::Windowed).unwrap();

        gfx.glGetnTexImage(GL_DYNAMIC_DRAW, 0, GL_RGBA, GL_UNSIGNED_INT, 0, std::ptr::null_mut());
    }
}
