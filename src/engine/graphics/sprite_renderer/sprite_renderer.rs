use gl46::{GL_DYNAMIC_DRAW, GL_RGBA, GL_SHADER_STORAGE_BUFFER, GL_TEXTURE_2D, GL_TEXTURE0, GL_TRIANGLES};
use gl_types::matrices::{Mat4, MatN};
use gl_types::{vec2, vec3, vec4};
use gl_types::vectors::{Vec2, Vec3, Vec4};

use crate::engine::graphics::image::Image;
use crate::engine::graphics::{embed_shader_source, BufferedMesh, FragmentShader, Graphics, Mesh, ShaderProgram, ShaderProgramBuilder, Texture, VBOBufferer, Vertex, VertexShader, UV};

use crate::engine::errors::Result;

const SSBO_OFFSET: isize = 16;

#[repr(align(16))]
#[derive(Clone, Copy, PartialEq)]
struct AlignedVec3(gl_types::vectors::Vec3);

impl std::fmt::Debug for AlignedVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
struct GLSpriteStruct {
    position: AlignedVec3,
    dimensions: Vec4,
    id: u32
}

impl Default for GLSpriteStruct {
    fn default() -> Self {
        Self {
            position: AlignedVec3(vec3!(0)),
            dimensions: vec4!(0),
            id: 0
        }
    }
}

#[derive(Clone, Copy)]
pub struct SpriteData {
    pub position: Vec3,
    pub anchor: Vec2,
    pub dimensions: Vec2,
    pub sprite_id: u32
}

pub struct SpriteRenderer {
    program: ShaderProgram,
    mesh: BufferedMesh,
    render_queue: Vec<GLSpriteStruct>,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    buffersize: usize,
    sprite_ssbo: u32,
    spritesheet_ssbo: u32,
    debug_ssbo: u32,
    sprite_sheet: Texture,
    sprite_map: Vec<Vec4>,
    view_location: i32,
    projection_location: i32,
    texel_offset_location: i32
}

impl SpriteRenderer {
    pub fn new(gfx: &Graphics, initial_buffer_size: usize, sprite_sheet: Image) -> Result<SpriteRenderer> {
        let mut program = ShaderProgramBuilder::new(gfx);
        
        let vertex_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite.vert");
        let fragment_shader_source = embed_shader_source!("src/engine/graphics/shaders/sprite.frag");

        let vert_shader = VertexShader::compile_shader(gfx, vertex_shader_source)?;
        let frag_shader = FragmentShader::compile_shader(gfx, fragment_shader_source)?; 

        program.attach_shader(vert_shader);
        program.attach_shader(frag_shader);

        let program = program.finish();

        gfx.glUseProgram(program.program());

        let view_location = gfx.glGetUniformLocation(program.program(), "view");
        let projection_location = gfx.glGetUniformLocation(program.program(), "projection");
        let texel_offset_location = gfx.glGetUniformLocation(program.program(), "texelOffset");

        println!("view: {}, projection: {}", view_location, projection_location);

        let sprite_sheet = sprite_sheet.as_texture(gfx, GL_RGBA);

        let vertex_data = Box::new([
            Vertex { x: 0.0, y: 0.0, z: 0.0 }, // bottom left
            Vertex { x: 1.0, y: 0.0, z: 0.0 },  // bottom right
            Vertex { x: 0.0, y: 1.0, z: 0.0 },  // top left
        
            Vertex { x: 0.0, y: 1.0, z: 0.0 },  // top left
            Vertex { x: 1.0, y: 0.0, z: 0.0 },  // bottom right
            Vertex { x: 1.0, y: 1.0, z: 0.0 },   // top right
        ]);

        let uv_data = Box::new([
            UV { u: 0.0 , v: 0.0 },
            UV { u: 1.0 , v: 0.0 },
            UV { u: 0.0 , v: 1.0 },

            UV { u: 0.0 , v: 1.0 },
            UV { u: 1.0 , v: 0.0 },
            UV { u: 1.0 , v: 1.0 },
        ]);

        let mesh = Mesh::new("Sprite Mesh".to_owned(), vertex_data, None, Some(uv_data), None, None);
        
        let mut vbo = VBOBufferer::new(gfx);
        let mesh = vbo.add_mesh(mesh);

        vbo.buffer_data(gfx);

        let mesh = mesh.take(); 
        
        let mut sprite_ssbo = 0;
        gfx.glGenBuffer(&mut sprite_ssbo);
        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, sprite_ssbo);
        gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, initial_buffer_size, GL_DYNAMIC_DRAW);
        gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, sprite_ssbo);

        let mut spritesheet_ssbo = 0;
        gfx.glGenBuffer(&mut spritesheet_ssbo);

        let debug_ssbo = 0;
        // gfx.glGenBuffer(&mut debug_ssbo);
        // gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, debug_ssbo);
        // gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, 32 * 4, GL_DYNAMIC_DRAW);
        // gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 4, debug_ssbo);

        Ok(SpriteRenderer { program, mesh, render_queue: Vec::new(), view_matrix: Mat4::IDENTITY, projection_matrix: Mat4::IDENTITY, buffersize: initial_buffer_size, sprite_ssbo, spritesheet_ssbo, debug_ssbo, sprite_sheet, sprite_map: Vec::new(), view_location, projection_location, texel_offset_location })
    }

    pub fn update_view_matrix(&mut self, view_matrix: Mat4) {
        self.view_matrix = view_matrix;
    }

    pub fn update_projection_matrix(&mut self, projection_matrix: Mat4) {
        self.projection_matrix = projection_matrix;
    }

    pub fn add_sprite(&mut self, x: u32, y: u32, width: u32, height: u32) -> usize {
        // Convert pixel coordinates to uv coordinates
        let wh = vec2!(self.sprite_sheet.width(), self.sprite_sheet.height());
        let v = vec4!(x, self.sprite_sheet.height() - y - height, width, height) / vec4!(wh, wh);

        let index = self.sprite_map.len();
        self.sprite_map.push(v);

        index
    }

    pub fn update_sprite_map(&self, gfx: &Graphics) {
        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, self.spritesheet_ssbo);
        gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, self.sprite_map.len() * size_of::<Vec4>() + SSBO_OFFSET as usize, GL_DYNAMIC_DRAW);
        gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 3, self.spritesheet_ssbo);

        // Buffer length data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, 0, &[self.sprite_map.len()]); 
        // Buffer sprite data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, SSBO_OFFSET, &self.sprite_map[..]); 

        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0);
    }

    fn buffer_sprite_data(&mut self, gfx: &Graphics) {
        let data_size = self.render_queue.len() * std::mem::size_of::<GLSpriteStruct>() + SSBO_OFFSET as usize;
        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, self.sprite_ssbo);

        if data_size > self.buffersize {
            // Multiply new szie by 50% to give some wiggle room
            self.buffersize = (data_size * 3) / 2;
            gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, self.buffersize, GL_DYNAMIC_DRAW);
            gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, self.sprite_ssbo);
        }

        // Buffer length data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, 0, &[self.render_queue.len()]); 
        // Buffer sprite data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, SSBO_OFFSET, &self.render_queue[..]); 
        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0);
    }

    pub fn queue_sprite_instance(&mut self, sprite: SpriteData) {
        let SpriteData { position, dimensions, anchor, sprite_id } = sprite;
        let dimensions = vec4!(anchor, dimensions);
        let position = AlignedVec3(position);
        let id = sprite_id;

        let sprite_data = GLSpriteStruct {
            position,
            dimensions,
            id
        };

        self.render_queue.push(sprite_data);
    }

    pub fn render(&mut self, gfx: &Graphics) {
        gfx.glBindVertexArray(self.mesh.vao());
        gfx.glUseProgram(self.program.program());
        self.buffer_sprite_data(gfx);
        gfx.glActiveTexture(GL_TEXTURE0);
        gfx.glBindTexture(GL_TEXTURE_2D, self.sprite_sheet.texture_id());

        let texel_offset = vec2!(1.0) / (vec2!(self.sprite_sheet.width(), self.sprite_sheet.height()) * 2.0);

        unsafe { gfx.glUniformMatrix4fv(self.view_location, 1, 0, self.view_matrix.as_slice()[0].as_ptr()) };
        unsafe { gfx.glUniformMatrix4fv(self.projection_location, 1, 0, self.projection_matrix.as_slice()[0].as_ptr()) };
        unsafe { gfx.glUniform2f(self.texel_offset_location, texel_offset.x(), texel_offset.y()) };
        // let mat = &debug[0..16];
        // let v1 = &debug[16..19];
        // let v2 = &debug[20..24];

        // println!("mat:\t{:?}\n\t{:?}\n\t{:?}\n\t{:?}", &mat[0..4], &mat[4..8], &mat[8..12], &mat[12..16]);
        // println!("v1: {:?}", v1);
        // println!("v2: {:?}\n", v2);

        gfx.glDrawArraysInstanced(GL_TRIANGLES, 0, self.mesh.len() as _, self.render_queue.len() as u32);
        self.render_queue.clear();

        // let mut debug = [0.0f32; 32];
        // gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, self.debug_ssbo);
        // unsafe { gfx.glGetBufferSubData(GL_SHADER_STORAGE_BUFFER, 0, 32 * 4, debug.as_mut_ptr() as _) };

        // println!("{:?}", debug);

    }
}

#[cfg(test)]
mod tests {
    use gl46::{GL_DYNAMIC_DRAW, GL_SHADER_STORAGE_BUFFER, GL_TRIANGLES};
    use gl_types::{vec3, vec4};
    
    impl PartialEq for GLSpriteStruct {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position && self.dimensions == other.dimensions && self.id == other.id
        }
    }

    use crate::engine::graphics::{embed_shader_source, image::Image, sprite_renderer::sprite_renderer::{AlignedVec3, GLSpriteStruct, SSBO_OFFSET}, FragmentShader, Graphics, ShaderProgramBuilder, VertexShader};

    use super::SpriteRenderer;

    #[test]
    pub fn sprite_struct_test() {
        let lock = crate::engine::graphics::test_lock::LOCK.lock().unwrap();

        let gfx = Graphics::init("test_window", 1289, 720, crate::engine::WindowMode::Windowed).unwrap();

        let renderer = SpriteRenderer::new(&gfx, 1024, Image::empty(0, 0)).unwrap();

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

        let sprite_structs = [GLSpriteStruct::default(); 2];

        let mut data_in = [GLSpriteStruct::default(); 2];

        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, ssbo);
        // Allocate space
        gfx.glBufferNull(GL_SHADER_STORAGE_BUFFER, std::mem::size_of::<GLSpriteStruct>() * sprite_structs.len() + SSBO_OFFSET as usize, GL_DYNAMIC_DRAW); 
        // Buffer length data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, 0, std::slice::from_ref(&sprite_structs.len())); 
        // Buffer sprite data
        gfx.glBufferSubData(GL_SHADER_STORAGE_BUFFER, SSBO_OFFSET,&sprite_structs); 
        gfx.glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 2, ssbo);

        gfx.glUseProgram(program.program());
        gfx.glBindVertexArray(renderer.mesh.vao());
        gfx.glDrawArrays(GL_TRIANGLES, 0, renderer.mesh.len() as _);

        unsafe { gfx.glGetBufferSubData(GL_SHADER_STORAGE_BUFFER, SSBO_OFFSET, std::mem::size_of::<GLSpriteStruct>() as isize * sprite_structs.len() as isize, data_in.as_mut_ptr() as *mut _) };

        println!("{:?}", data_in); 

        gfx.glBindBuffer(GL_SHADER_STORAGE_BUFFER, 0); // unbind
        
        let expected = [
            GLSpriteStruct {
                position: AlignedVec3(vec3!(1, 2, 3)),
                dimensions: vec4!(4, 5, 6, 7),
                id: 8
            },
            GLSpriteStruct {
                position: AlignedVec3(vec3!(9, 10, 11)),
                dimensions: vec4!(12, 13, 14, 15),
                id: 16
            },
        ];

        assert_eq!(data_in, expected);
        drop(gfx);
        drop(lock);
    }
}
